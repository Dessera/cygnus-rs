pub mod auth;
pub mod interface;

use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::sync::Mutex;

use crate::{
  common::LogLevel, component::Component, context::Context, error::JludResult,
};

#[derive(Parser, Debug)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,

  /// Log level
  #[arg(value_enum, short, long, default_value = "info")]
  pub log_level: LogLevel,

  /// Specify the path to the configuration file
  #[arg(short, long)]
  pub config: Option<String>,

  /// Specify the path to the user data file (No effect when save_user is false)
  #[arg(short, long)]
  pub user: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Run the authentication client
  Auth(auth::Auth),
}

impl Component for Cli {
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    match &mut self.command {
      Command::Auth(auth) => auth.run(context).await,
    }
  }
}
