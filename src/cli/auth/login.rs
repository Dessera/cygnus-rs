use std::sync::Arc;

use pnet::datalink;
use rand::random;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::{
  common::{
    data::{checksum, ror},
    host::get_host_name,
  },
  component::Component,
  context::Context,
  error::{JludError, JludResult},
};

#[derive(Debug)]
pub struct LoginSender;

impl LoginSender {
  pub fn new() -> Self {
    Self {}
  }

  fn get_login_data(
    username: &str,
    password: &str,
    salt: &[u8; 4],
    client_ip: &[u8; 4],
    mac: &[u8; 6],
    md5a: &mut [u8; 16],
  ) -> Vec<u8> {
    let mut data = vec![0u8; 349];

    let password_len = if password.len() > 16 {
      16
    } else {
      password.len()
    };

    data[0] = 0x03;
    data[1] = 0x01;
    data[2] = 0x00;
    data[3] = username.len() as u8 + 20;

    md5a.copy_from_slice(
      &md5::compute(
        [0x03, 0x01]
          .iter()
          .chain(salt.iter())
          .chain(password.as_bytes())
          .copied()
          .collect::<Vec<u8>>(),
      )
      .0,
    );
    data[4..20].copy_from_slice(md5a);

    let mut username_data = username.as_bytes().to_vec();
    username_data.resize(36, 0);
    data[20..56].copy_from_slice(&username_data);

    data[56] = 0x20;
    data[57] = 0x05;

    for i in 0..6 {
      data[58 + i] = md5a[i] ^ mac[i];
    }

    let md5b = md5::compute(
      [0x01]
        .iter()
        .chain(password.as_bytes())
        .chain(salt.iter())
        .chain([0x00; 4].iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[64..80].copy_from_slice(&md5b.0);

    data[80] = 0x01;

    data[81..85].copy_from_slice(client_ip);
    data[85..97].copy_from_slice(&[0x00; 12]);

    let md5c = md5::compute(
      [].iter()
        .chain(&data[0..97])
        .chain([0x14, 0x00, 0x07, 0x0b].iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[97..105].copy_from_slice(&md5c.0[0..8]);

    data[105] = 0x01;

    let mut hostname_data = get_host_name().as_bytes().to_vec();
    hostname_data.resize(32, 0);
    data[110..142].copy_from_slice(&hostname_data);

    // TODO: Primary DNS
    data[142..146].copy_from_slice(&[10; 4]);

    // TODO: DHCP Server
    data[146..150].copy_from_slice(&[10; 4]);

    // TODO: Secondary DNS
    data[150..154].copy_from_slice(&[10; 4]);

    // delimiter to 162
    // TODO: Os data
    data[162] = 0x94; //unknown 162+4=166
    data[166] = 0x06; //os major 166+4=170
    data[170] = 0x02; //os minor 170+4=174
    data[174] = 0xf0; //os build
    data[175] = 0x23; //os build 174+4=178
    data[178] = 0x02; //os unknown 178+4=182

    //DRCOM CHECK
    data[182] = 0x44;
    data[183] = 0x72;
    data[184] = 0x43;
    data[185] = 0x4f;
    data[186] = 0x4d;
    data[187] = 0x00;
    data[188] = 0xcf;
    data[189] = 0x07;
    data[190] = 0x6a;

    // 0x00 to 246
    data[246..286]
      .copy_from_slice("1c210c99585fd22ad03d35c956911aeec1eb449b".as_bytes());
    // 0x00 to 310
    data[310] = 0x6a;
    data[313] = password_len as u8;

    let ror_data = ror(md5a, password.as_bytes());

    data[314..password_len + 314].copy_from_slice(&ror_data[0..password_len]);
    data[password_len + 314] = 0x02;
    data[password_len + 315] = 0x0c;

    let checksum_val = checksum(
      &[0x01, 0x26, 0x07, 0x11, 0x00, 0x00]
        .iter()
        .chain(mac.iter())
        .copied()
        .collect::<Vec<u8>>(),
    );

    data[password_len + 316..password_len + 320].copy_from_slice(&checksum_val);
    data[password_len + 320] = 0x00;
    data[password_len + 321] = 0x00;

    data[password_len + 322..password_len + 328].copy_from_slice(mac);

    let zero_count = (4 - password_len % 4) % 4;
    for i in 0..zero_count {
      data[password_len + 328 + i] = 0x00;
    }

    // random 2 bytes
    data[password_len + 328 + zero_count] = random();
    data[password_len + 329 + zero_count] = random();

    let new_len = 334 + (password_len - 1) / 4 * 4;
    data.resize(new_len, 0x00);

    data
  }
}

impl Component for LoginSender {
  #[tracing::instrument(skip_all, name = "login_sender")]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let mut ctx = context.lock().await;

    info!("Starting login");

    let username = ctx.user.username.clone();
    let password = ctx.user.password.clone();
    let interface = ctx.interface.config.interface.clone();
    let interfaces = datalink::interfaces();
    let target = interfaces
      .iter()
      .find(|iface| iface.name == interface)
      .ok_or_else(|| JludError::InterfaceNotFound(interface.to_string()))?;
    let mac = target
      .mac
      .ok_or_else(|| JludError::MissingField("mac".to_string()))?;
    let salt = ctx.common.salt.clone();
    let client_ip = ctx.common.client_ip.clone();
    let data = LoginSender::get_login_data(
      &username,
      &password,
      &salt,
      &client_ip,
      &[mac.0, mac.1, mac.2, mac.3, mac.4, mac.5],
      &mut ctx.common.md5a,
    );

    ctx.interface.send(&data).await?;

    let mut recv_buf = [0; 48];
    ctx.interface.recv_with_timeout(&mut recv_buf).await?;

    if recv_buf[0] == 0x04 {
      info!("Login success");
      ctx.common.tail.copy_from_slice(&recv_buf[23..39]);
      Ok(())
    } else if recv_buf[0] == 0x05 && recv_buf[4] == 0x0b {
      // invalid mac
      error!("Login failed: invalid mac");
      Err(JludError::InvalidMac)
    } else if recv_buf[0] == 0x05 {
      error!("Login failed: invalid username or password");
      Err(JludError::InvalidUsernameOrPassword)
    } else {
      error!("Login failed: unknown error");
      Err(JludError::UnknownError)
    }
  }
}
