use std::sync::Arc;

use rand::random;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::{
  component::Component,
  context::Context,
  error::{JludError, JludResult},
};

#[derive(Debug)]
pub struct Challenger;

impl Challenger {
  pub fn new() -> Self {
    Self {}
  }

  fn get_challenge_data(try_times: i32) -> Vec<u8> {
    let mut data = vec![0; 20];
    data[0] = 0x01;
    data[1] = 0x02 + (try_times as u8);
    data[2] = random();
    data[3] = random();
    data[4] = 0x6a;
    data
  }
}

impl Component for Challenger {
  #[tracing::instrument(skip_all)]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let mut ctx = context.lock().await;
    let mut try_times = 0;

    let mut recv_buf = [0; 32];

    info!("Starting challenge");

    while try_times < 5 {
      let data = Challenger::get_challenge_data(try_times);
      match ctx.interface.send(&data).await {
        Ok(_) => {}
        Err(e) => {
          warn!("Failed to send challenge data: {}", e);
          try_times += 1;
          continue;
        }
      };
      match ctx.interface.recv_with_timeout(&mut recv_buf).await {
        Ok(_) => {}
        Err(e) => {
          warn!("Failed to receive challenge data: {}", e);
          try_times += 1;
          continue;
        }
      }

      if recv_buf[0] == 0x02 {
        ctx.common.salt.copy_from_slice(&recv_buf[4..8]);
        ctx.common.client_ip.copy_from_slice(&recv_buf[20..24]);
        info!("Challenge completed");
        return Ok(());
      }

      warn!("Challenge failed, retrying");
      try_times += 1;
    }

    error!("Failed to complete challenge");
    Err(JludError::ChallengeFailed)
  }
}
