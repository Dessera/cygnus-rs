#[macro_use]
extern crate dotenv_codegen;

use app_lib::{
  cli::Cli, context::ContextConfig, utils::path::app_config_file, AppError,
};
use clap::Parser;
use dotenv::dotenv;
use tracing::Level;
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

  // Determine configuration file path
  let config_path =
    app_config_file().expect("Could not determine configuration file path");
  let config = ContextConfig::try_from_file(config_path)
    .expect("Could not read configuration file");

  app_lib::run(args).await
}
