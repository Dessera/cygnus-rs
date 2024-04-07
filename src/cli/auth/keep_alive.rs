use std::sync::Arc;

use rand::random;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
  common::data::crc, component::Component, context::Context, error::JludResult,
};

pub enum AliveType {
  FIRST,
  SECOND,
  EXTRA,
}

#[derive(Debug)]
pub struct KeepAliveSender {
  keep_40_count: i32,
}

impl KeepAliveSender {
  pub fn new() -> Self {
    Self { keep_40_count: 0 }
  }

  fn get_keep_alive_data_38(md5a: &[u8; 16], tail: &[u8; 16]) -> Vec<u8> {
    let mut data = vec![0u8; 38];
    data[0] = 0xff;
    data[1..17].copy_from_slice(md5a);
    data[20..36].copy_from_slice(tail);
    data[36] = random();
    data[37] = random();
    data
  }

  fn get_keep_alive_data_40(
    &mut self,
    alive_type: AliveType,
    tail_2: &[u8; 4],
    client_ip: &[u8; 4],
    keep_alive_version: &(u8, u8),
  ) -> Vec<u8> {
    let mut data = vec![0u8; 40];

    data[0] = 0x07;
    data[1] = self.keep_40_count as u8;

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
      data[6] = keep_alive_version.0;
      data[7] = keep_alive_version.1;
    }

    data[8] = random();
    data[9] = random();
    data[16..20].copy_from_slice(tail_2);

    if let AliveType::SECOND = alive_type {
      let tmp = crc(
        &data[0..24]
          .iter()
          .chain(client_ip)
          .copied()
          .collect::<Vec<u8>>(),
      );
      data[24..28].copy_from_slice(&tmp);
      data[28..32].copy_from_slice(client_ip);
    }

    self.keep_40_count += 1;
    if self.keep_40_count > 255 {
      self.keep_40_count = 0;
    }

    data
  }
}

impl Component for KeepAliveSender {
  #[tracing::instrument(skip_all, name = "keep_alive_sender")]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let mut ctx = context.lock().await;

    let mut buf = [0; 32];

    let keep_38 = KeepAliveSender::get_keep_alive_data_38(
      &ctx.common.md5a,
      &ctx.common.tail,
    );
    ctx.interface.send(&keep_38).await?;
    ctx.interface.recv_with_timeout(&mut buf).await?;
    ctx.common.keep_alive_version = (buf[28], buf[29]);

    if self.keep_40_count % 21 == 0 {
      let keep_40 = self.get_keep_alive_data_40(
        AliveType::EXTRA,
        &ctx.common.tail_2,
        &ctx.common.client_ip,
        &ctx.common.keep_alive_version,
      );
      ctx.interface.send(&keep_40).await?;
      ctx.interface.recv_with_timeout(&mut buf).await?;
      info!("Keep alive extra accepted");
    }

    let keep_40 = self.get_keep_alive_data_40(
      AliveType::FIRST,
      &ctx.common.tail_2,
      &ctx.common.client_ip,
      &ctx.common.keep_alive_version,
    );
    ctx.interface.send(&keep_40).await?;
    ctx.interface.recv_with_timeout(&mut buf).await?;
    ctx.common.tail_2.copy_from_slice(&buf[16..20]);
    info!("Keep alive first accepted");

    let keep_40 = self.get_keep_alive_data_40(
      AliveType::SECOND,
      &ctx.common.tail_2,
      &ctx.common.client_ip,
      &ctx.common.keep_alive_version,
    );
    ctx.interface.send(&keep_40).await?;
    ctx.interface.recv_with_timeout(&mut buf).await?;
    info!("Keep alive second accepted");

    Ok(())
  }
}
