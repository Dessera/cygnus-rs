use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,

  /// Log level
  #[arg(value_enum, short, long, default_value = "info")]
  pub log_level: LogLevel,

  // default to app default config directory
  /// Specify the path to the configuration file
  #[arg(short, long, default_value = "config.toml")]
  pub config: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl Into<tracing::Level> for LogLevel {
  fn into(self) -> tracing::Level {
    match self {
      LogLevel::Trace => tracing::Level::TRACE,
      LogLevel::Debug => tracing::Level::DEBUG,
      LogLevel::Info => tracing::Level::INFO,
      LogLevel::Warn => tracing::Level::WARN,
      LogLevel::Error => tracing::Level::ERROR,
    }
  }
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Run the authentication client
  Auth {},
}
