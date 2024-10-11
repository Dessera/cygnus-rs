use crate::user::error::UserError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
  #[error("IO error -> {0}")]
  Io(#[from] std::io::Error),

  #[error("User error -> {0}")]
  User(#[from] UserError),

  #[error("Challenge max tries exceeded")]
  ChallengeMaxTriesExceeded,

  #[error("App max tries exceeded")]
  AppMaxTriesExceeded,

  #[error("Invalid MAC address")]
  InvalidMacAddress,

  #[error("Invalid username or password")]
  InvalidUsernameOrPassword,

  #[error("Unknown error")]
  Unknown,
}

pub type AuthResult<T> = Result<T, AuthError>;
