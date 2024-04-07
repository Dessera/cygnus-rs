#[allow(unused_imports)]
#[macro_use]
extern crate dotenv_codegen;

use std::sync::Arc;

use app_lib::{
  cli::Cli, common::path::JludPath, component::Component, config::Config,
  context::Context, error::JludError,
};
use clap::Parser;
use directories::ProjectDirs;
use dotenv::dotenv;
use tokio::sync::Mutex;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), JludError> {
  dotenv().ok();
  let mut cli = Cli::parse();

  let mut jlud_path: JludPath =
    match ProjectDirs::from("com", "Dessera", dotenv!("JLUD_APP_NAME")) {
      Some(dirs) => dirs.into(),
      None => {
        warn!("Failed to get base project directories, using default paths");
        JludPath::new(
          dotenv!("JLUD_APP_CONFIG_FILE").into(),
          dotenv!("JLUD_APP_PASSWORD_FILE").into(),
        )
      }
    };
  if let Some(path) = cli.config.clone() {
    jlud_path.config = path.into();
  }
  if let Some(path) = cli.user.clone() {
    jlud_path.user = path.into();
  }

  // Logger initialization
  let log_level: Level = cli.log_level.into();
  let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

  info!(
    "Using config file: {}",
    jlud_path.config.to_string_lossy().to_string()
  );

  let config = Config::try_from_file(&jlud_path.config)
    .await
    .unwrap_or_else(|e| {
      warn!("Failed to load config file: {}, using default config", e);
      Config::default()
    });
  info!("Config created");

  // Create context
  let ctx = match Context::create(config, jlud_path).await {
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
