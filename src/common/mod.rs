use clap::ValueEnum;
use tracing::Level;

pub mod components;
pub mod host;
pub mod path;
pub mod data;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl Into<Level> for LogLevel {
  fn into(self) -> Level {
    match self {
      LogLevel::Trace => Level::TRACE,
      LogLevel::Debug => Level::DEBUG,
      LogLevel::Info => Level::INFO,
      LogLevel::Warn => Level::WARN,
      LogLevel::Error => Level::ERROR,
    }
  }
}
