pub use clap::{Parser, Subcommand};

use crate::auth::args::AuthArgs;
use crate::user::args::UserArgs;

#[derive(Parser)]
pub struct Args {
  #[command(subcommand)]
  pub command: ArgsCommand,
}

#[derive(Subcommand)]
pub enum ArgsCommand {
  /// Authenticate a user
  Auth(AuthArgs),

  /// Operate on user authentication files
  User(UserArgs),
}
