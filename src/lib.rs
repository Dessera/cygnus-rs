pub mod cli;
pub mod component;
pub mod context;
pub mod utils;

use std::sync::Arc;

use cli::{Cli, Command};
use context::{Context, ContextConfig};
use tokio::sync::Mutex;
use tracing::{event, Level};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
  #[error("{0}")]
  ContextError(#[from] context::ContextError),
}

#[tracing::instrument(skip(args, config))]
pub async fn run(args: Cli, config: ContextConfig) -> Result<(), AppError> {
  let context = Arc::new(Mutex::new(Context::new(config)));
  event!(Level::INFO, "Created context");

  match &args.command {
    Command::Auth {} => {}
  }

  Ok(())
}
