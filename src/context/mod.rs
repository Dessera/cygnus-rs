use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use serde::{Deserialize, Serialize};

use crate::utils::path::app_config_file;

use self::password::{PasswordStore, PasswordStoreConfig, PasswordStoreError};
pub mod password;

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
  #[error("{0}")]
  PasswordStoreError(#[from] PasswordStoreError),

  #[error("Unable to read configuration file: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Unable to parse configuration file: {0}")]
  TomlError(#[from] toml::de::Error),

  // from option unable to determine configuration file path
  #[error("Unable to determine configuration file path")]
  ConfigPathError,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContextConfig {
  pub password: PasswordStoreConfig,
}

impl ContextConfig {
  pub async fn try_from_file(
    path: PathBuf,
  ) -> Result<ContextConfig, ContextError> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: ContextConfig = toml::from_str(&contents)?;
    Ok(config)
  }

  pub async fn try_from_config_file() -> Result<ContextConfig, ContextError> {
    let path = app_config_file().ok_or(ContextError::ConfigPathError)?;
    Self::try_from_file(path).await
  }
}

#[derive(Debug, Default)]
pub struct Context {
  password_store: PasswordStore,
}

impl Context {
  pub fn new(config: ContextConfig) -> Context {
    Context {
      password_store: PasswordStore::new(config.password),
    }
  }
}
