use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::{component::Component, context::Context, error::JludResult};

#[derive(Debug)]
pub struct UserInfoCollector;

impl UserInfoCollector {
  pub fn new() -> Self {
    Self {}
  }
}

impl Component for UserInfoCollector {
  #[tracing::instrument(skip_all, name = "user_info_collector")]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let ctx = &mut context.lock().await;
    let user_path = ctx.path.user.clone();

    info!("Starting user info collection");

    if ctx.user.config.save_user {
      info!("Loading user info from passwd file");
      if let Err(e) = ctx.user.load_from_config(&user_path).await {
        warn!(
          "Could not read user info from config file: {}, prompting user for info", e
        );
        ctx.user.read_user_info_from_prompt()?;
        if let Err(e) = ctx.user.save_to_config(&user_path).await {
          warn!("Failed to save user info to user file: {}", e);
        }
      }
    } else {
      // prompt user for info
      info!("Prompting user for info");
      ctx.user.read_user_info_from_prompt()?;
    }

    info!("User {:?} info collected", ctx.user.username);

    Ok(())
  }
}
