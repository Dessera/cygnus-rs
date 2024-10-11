pub mod args;
pub mod cipher;
pub mod data;
pub mod error;

pub use data::User;

use args::{UserArgs, UserCommand};
use cipher::UserCipher;
use error::UserResult;
use std::fs::OpenOptions;

pub fn user_command_resolver(args: UserArgs) -> UserResult<()> {
  match args.command {
    UserCommand::Create(create_args) => {
      let fd = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&create_args.file)?;
      let mac = User::transform_mac(&create_args.mac)?;
      let user = User::new(create_args.username, create_args.password, mac);
      UserCipher::encrypt(fd, user)?;
      println!("User file created: {}", create_args.file);
    }
    UserCommand::Inspect(inspect_args) => {
      let fd = OpenOptions::new().read(true).open(&inspect_args.file)?;
      let user = UserCipher::decrypt(fd)?;
      println!("Username: {}", user.username);
    }
  }
  Ok(())
}
