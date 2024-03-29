#[macro_use]
extern crate dotenv_codegen;

use app_lib::{cli::Cli, context::ContextConfig, AppError};
use clap::Parser;
use dotenv::dotenv;
use tracing::{warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), AppError> {
  // Load environment variables from .env file
  dotenv().ok();

  // Parse command line arguments
  let args = Cli::parse();

  // Set up logging
  let log_level: Level = args.log_level.into();
  let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

  let config =
    ContextConfig::try_from_config_file()
      .await
      .unwrap_or_else(|e| {
        warn!("{}, using defaults", e);
        ContextConfig::default()
      });

  app_lib::run(args, config).await
}
