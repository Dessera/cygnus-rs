pub mod cli;
pub mod context;
pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
  // #[error("{0}")]
}

pub async fn run(args: cli::Cli) -> Result<(), AppError> {
  // let args = cli::Cli::parse();

  Ok(())
}
