use std::net::UdpSocket;

use crate::user::User;

use super::{
  data::{self, AliveType, DrContextData},
  error::AuthResult,
};

pub struct DrContext {
  pub client: UdpSocket,
  pub data: DrContextData,
  pub user: User,
}

impl DrContext {
  pub fn try_new(user: User, timeout: u64) -> AuthResult<Self> {
    let client = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = std::time::Duration::from_secs(timeout);
    client.connect("10.100.61.3:61440")?;
    client.set_read_timeout(Some(timeout))?;
    client.set_write_timeout(Some(timeout))?;
    let data = DrContextData::default();

    Ok(Self { client, data, user })
  }
}

impl DrContext {
  pub fn get_challenge_data(&self, try_times: u8, data: &mut [u8; 20]) {
    data[0] = 0x01;
    data[1] = 0x02 + try_times;
    data[2] = rand::random();
    data[3] = rand::random();
    data[4] = 0x6a;
  }

  pub fn get_login_data(&mut self, data: &mut Vec<u8>) {
    let password_len = match self.user.password.len() {
      len if len > 16 => 16,
      len => len,
    };
    data[0..3].copy_from_slice(&[0x03, 0x01, 0x00]);
    data[3] = self.user.username.len() as u8 + 20;

    let md5a = md5::compute(
      [0x03, 0x01]
        .iter()
        .chain(self.data.salt.iter())
        .chain(self.user.password.as_bytes())
        .copied()
        .collect::<Vec<u8>>(),
    );
    self.data.md5a.copy_from_slice(&md5a.0);
    data[4..20].copy_from_slice(&self.data.md5a);

    let mut username_data = self.user.username.as_bytes().to_vec();
    username_data.resize(36, 0);
    data[20..56].copy_from_slice(&username_data);

    data[56..58].copy_from_slice(&[0x20, 0x05]);

    for i in 0..6 {
      data[58 + i] = md5a.0[i] ^ self.user.mac[i];
    }

    let md5b = md5::compute(
      [0x01]
        .iter()
        .chain(self.user.password.as_bytes())
        .chain(self.data.salt.iter())
        .chain([0x00; 4].iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[64..80].copy_from_slice(&md5b.0);

    data[80] = 0x01;

    data[81..85].copy_from_slice(&self.data.client_ip);
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

    let mut hostname_data = Self::get_host_name().as_bytes().to_vec();
    hostname_data.resize(32, 0);
    data[110..142].copy_from_slice(&hostname_data);

    data[142..146].copy_from_slice(&[10; 4]);
    data[146..150].copy_from_slice(&[10; 4]);
    data[150..154].copy_from_slice(&[10; 4]);

    data[162] = 0x94;
    data[166] = 0x06;
    data[170] = 0x02;
    data[174] = 0xf0;
    data[175] = 0x23;
    data[178] = 0x02;

    data[182..191]
      .copy_from_slice(&[0x44, 0x72, 0x43, 0x4f, 0x4d, 0x00, 0xcf, 0x07, 0x6a]);

    data[246..286]
      .copy_from_slice("1c210c99585fd22ad03d35c956911aeec1eb449b".as_bytes());

    data[310] = 0x6a;
    data[313] = password_len as u8;

    let ror_data = ror(md5a.as_slice(), self.user.password.as_bytes());

    data[314..password_len + 314].copy_from_slice(&ror_data[0..password_len]);
    data[password_len + 314] = 0x02;
    data[password_len + 315] = 0x0c;

    let checksum_val = checksum(
      &[0x01, 0x26, 0x07, 0x11, 0x00, 0x00]
        .iter()
        .chain(self.user.mac.iter())
        .copied()
        .collect::<Vec<u8>>(),
    );

    data[password_len + 316..password_len + 320].copy_from_slice(&checksum_val);
    data[password_len + 320] = 0x00;
    data[password_len + 321] = 0x00;

    data[password_len + 322..password_len + 328]
      .copy_from_slice(&self.user.mac);

    let zero_count = (4 - password_len % 4) % 4;
    for i in 0..zero_count {
      data[password_len + 328 + i] = 0x00;
    }

    data[password_len + 328 + zero_count] = rand::random();
    data[password_len + 329 + zero_count] = rand::random();

    // let new_len = 334 + (password_len - 1) / 4 * 4;
    let new_len = 334 + password_len - 1;
    data.resize(new_len, 0);
  }

  pub fn get_keep_alive_data_38(&self, data: &mut [u8; 38]) {
    data[0] = 0xff;
    data[1..17].copy_from_slice(&self.data.md5a);
    data[20..36].copy_from_slice(&self.data.tail);
    data[36] = rand::random();
    data[37] = rand::random();
  }

  pub fn get_keep_alive_data_40(
    &mut self,
    alive_type: AliveType,
    keep_40_count: u8,
    data: &mut [u8; 40],
  ) {
    data[0] = 0x07;
    data[1] = keep_40_count;
    data[2] = 0x20;
    data[3] = 0x00;
    data[4] = 0x0b;
    data[5] = match alive_type {
      AliveType::FIRST | AliveType::EXTRA => 0x01,
      AliveType::SECOND => 0x03,
    };
    if let AliveType::EXTRA = alive_type {
      data[6] = 0x0f;
      data[7] = 0x27;
    } else {
      data[6] = self.data.keep_alive_version.0;
      data[7] = self.data.keep_alive_version.1;
    }
    data[8] = rand::random();
    data[9] = rand::random();
    data[16..20].copy_from_slice(&self.data.tail_2);
    if let data::AliveType::SECOND = alive_type {
      let tmp = crc(
        &data[0..24]
          .iter()
          .chain(self.data.client_ip.iter())
          .copied()
          .collect::<Vec<u8>>(),
      );
      data[24..28].copy_from_slice(&tmp);
      data[28..32].copy_from_slice(&self.data.client_ip);
    }
  }

  pub fn get_host_name() -> String {
    match hostname::get() {
      Ok(host) => host.to_string_lossy().to_string(),
      Err(_) => "unknown".to_string(),
    }
  }
}

fn ror(data: &[u8], pwd: &[u8]) -> Vec<u8> {
  let mut ret = Vec::new();
  for i in 0..pwd.len() {
    let x = data[i] ^ pwd[i];
    ret.push((x << 3) + (x >> 5));
  }
  ret
}

fn checksum(data: &[u8]) -> [u8; 4] {
  let mut sum = [0u8; 4];
  let len = data.len();
  let mut i = 0;
  while i + 3 < len {
    sum[0] ^= data[i + 3];
    sum[1] ^= data[i + 2];
    sum[2] ^= data[i + 1];
    sum[3] ^= data[i];
    i += 4;
  }
  if i < len {
    let mut tmp = [0u8; 4];
    for j in (0..4).rev() {
      tmp[j] = data[i];
      i += 1;
    }
    for j in 0..4 {
      sum[j] ^= tmp[j];
    }
  }
  let mut big_integer = u32::from_le_bytes(sum) as u64;
  big_integer *= 1968;
  let bytes = big_integer.to_le_bytes();
  let mut i = 0;
  let mut ret = [0u8; 4];
  for j in (0..4).rev() {
    ret[j] = bytes[i];
    i += 1;
  }
  ret
}

fn crc(data: &[u8]) -> [u8; 4] {
  let mut sum: u32 = 0;
  let len = data.len();
  let mut i = 0;

  while i + 1 < len {
    let byte1 = data[i + 1] as u32;
    let byte2 = data[i] as u32;
    sum ^= byte1 << 8 | byte2;
    i += 2;
  }

  let mut result: [u8; 4] = [0; 4];
  result[0] = (sum & 0xFF) as u8;
  result[1] = ((sum >> 8) & 0xFF) as u8;
  result[2] = ((sum >> 16) & 0xFF) as u8;
  result[3] = ((sum >> 24) & 0xFF) as u8;

  result
}
