pub mod common;
pub mod interface;
pub mod user;

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::error::JludResult;

use self::{
  common::CommonConfig, interface::InterfaceConfig, user::UserConfig,
};

/// Application configurations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  /// Common configurations
  #[serde(default)]
  pub common: CommonConfig,

  /// User configurations
  #[serde(default)]
  pub user: UserConfig,

  /// Interface configurations
  #[serde(default)]
  pub interface: InterfaceConfig,
}

impl Config {
  pub async fn try_from_file(path: &str) -> JludResult<Self> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
  }
}
