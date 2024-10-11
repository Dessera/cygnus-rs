use cygnus::{
  args::{Args, ArgsCommand, Parser},
  auth::auth_command_resolver,
  user::user_command_resolver,
};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
  let args = Args::parse();

  match args.command {
    ArgsCommand::User(usr_args) => {
      user_command_resolver(usr_args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
      });
    }
    ArgsCommand::Auth(auth_args) => {
      let log_level: Level = auth_args.log_level.clone().into();
      let subscriber =
        FmtSubscriber::builder().with_max_level(log_level).finish();
      tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
        eprintln!("Failed to set default subscriber: {}", e);
        eprintln!("App will continue without logging");
      });
      auth_command_resolver(auth_args).unwrap_or_else(|e| {
        error!("Error when running auth command: {}", e);
        std::process::exit(1);
      });
    }
  }
}
