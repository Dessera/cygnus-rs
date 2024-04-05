#[allow(unused_imports)]
#[macro_use]
extern crate dotenv_codegen;

use std::sync::Arc;

use app_lib::{
  cli::Cli, common::path::app_config_file, component::Component,
  config::Config, context::Context, error::JludError,
};
use clap::Parser;
use dotenv::dotenv;
use tokio::sync::Mutex;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), JludError> {
  dotenv().ok();
  let mut cli = Cli::parse();

  let log_level: Level = cli.log_level.into();
  let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

  let config_path = match cli.config.clone() {
    Some(path) => path,
    _ => match app_config_file() {
      Some(path) => path.to_string_lossy().to_string(),

      // in most cases, this will not be used
      None => dotenv!("JLUD_APP_CONFIG_FILE").to_string(),
    },
  };
  info!("Using config file: {}", config_path);

  let config = Config::try_from_file(&config_path)
    .await
    .unwrap_or_else(|e| {
      warn!("Failed to load config file: {}, using default config", e);
      Config::default()
    });
  info!("Config created");

  let ctx = match Context::create(config).await {
    Ok(ctx) => ctx,
    Err(e) => {
      error!("Failed to create context: {}", e);
      return Err(e);
    }
  };
  info!("Context created");

  match cli.run(Arc::new(Mutex::new(ctx))).await {
    Ok(_) => info!("Command completed successfully"),
    Err(e) => {
      error!("Command failed: {}", e);
      return Err(e);
    }
  }

  Ok(())
}
