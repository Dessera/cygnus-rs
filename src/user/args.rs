use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct UserArgs {
  #[command(subcommand)]
  pub command: UserCommand,
}

#[derive(Subcommand)]
pub enum UserCommand {
  /// Create a new user authentication file
  Create(UserCreateArgs),

  /// Inspect an existing user authentication file
  Inspect(UserInspectArgs),
}

#[derive(Parser)]
pub struct UserCreateArgs {
  /// The username to create
  #[arg(short, long)]
  pub username: String,

  /// The password to use
  #[arg(short, long)]
  pub password: String,

  /// The MAC address to use
  #[arg(short, long)]
  pub mac: String,

  /// The file to write the user authentication to
  #[arg(short, long)]
  pub file: String,
}

#[derive(Parser)]
pub struct UserInspectArgs {
  /// The user authentication file to inspect
  #[arg(short, long)]
  pub file: String,
}
