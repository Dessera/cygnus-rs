use aes_gcm::aead::Error as AeadError;

#[derive(thiserror::Error, Debug)]
pub enum UserError {
  #[error("IO error -> {0}")]
  Io(#[from] std::io::Error),

  #[error("Aead error -> {0}")]
  Aead(#[from] AeadError),

  #[error("Invalid UTF-8 -> {0}")]
  Utf8(#[from] std::string::FromUtf8Error),

  #[error("Invalid MAC address -> {0}")]
  Mac(#[from] std::num::ParseIntError),
}

pub type UserResult<T> = Result<T, UserError>;
