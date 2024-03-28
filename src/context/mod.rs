use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use self::password::{PasswordStoreConfig, PasswordStoreError};
pub mod password;

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
  #[error("{0}")]
  PasswordStoreError(#[from] PasswordStoreError),

  #[error("Unable to read configuration file: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Unable to parse configuration file: {0}")]
  TomlError(#[from] toml::de::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextConfig {
  pub password: PasswordStoreConfig,
}

impl ContextConfig {
  pub fn try_from_file(path: PathBuf) -> Result<ContextConfig, ContextError> {
    let config_str = std::fs::read_to_string(path)?;
    let config: ContextConfig = toml::from_str(&config_str)?;
    Ok(config)
  }
}

#[derive(Debug)]
pub struct Context {
  password_store: password::PasswordStore,
}

impl Context {
  pub fn new(config: ContextConfig) -> Context {
    Context {
      password_store: password::PasswordStore::new(config.password),
    }
  }
}
