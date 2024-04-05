use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::{
  component::Component,
  context::Context,
  error::{JludError, JludResult},
};

#[derive(Debug)]
pub struct UserInfoCollector;

impl UserInfoCollector {
  pub fn new() -> Self {
    Self {}
  }
}

impl Component for UserInfoCollector {
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let user_ctx = &mut context.lock().await.user;
    if user_ctx.config.save_user {
      // read user info from config file
      match user_ctx.load_from_config().await {
        Ok(_) => {}
        Err(e) => match e {
          JludError::Io(_) => {
            // prompt user for info
            warn!("Could not read user info from config file, prompting user for info");
            user_ctx.read_user_info_from_prompt()?;
            user_ctx.save_to_config().await?;
          }
          _ => return Err(e),
        },
      }
    } else {
      // prompt user for info
      user_ctx.read_user_info_from_prompt()?;
    }

    if let Some(ref username) = user_ctx.username {
      info!("User {:?} info collected", username);
    }

    Ok(())
  }
}
