#[derive(thiserror::Error, Debug)]
pub enum JludError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("TOML error: {0}")]
  Toml(#[from] toml::de::Error),

  #[error("AES-GCM error: {0}")]
  AesGcm(#[from] aes_gcm::Error),

  #[error("FromUtf8 error: {0}")]
  FromUtf8(#[from] std::string::FromUtf8Error),

  #[error("Could not find password file")]
  PasswordFileNotFound,

  #[error("Unsupported platform")]
  UnsupportedPlatform,

  #[error("Request timeout")]
  Timeout,

  #[error("Challenge failed")]
  ChallengeFailed,

  #[error("Missing required field: {0}")]
  MissingField(String),

  #[error("Could not find interface: {0}")]
  InterfaceNotFound(String),

  #[error("Invalid MAC address")]
  InvalidMac,

  #[error("Invalid username or password")]
  InvalidUsernameOrPassword,

  #[error("Unknown error")]
  UnknownError,
}

pub type JludResult<T> = Result<T, JludError>;
