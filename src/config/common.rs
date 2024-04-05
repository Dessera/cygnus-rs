use serde::{Deserialize, Serialize};

/// Common configurations
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonConfig {
  /// Set retry times, None means infinite
  #[serde(default)]
  pub retry: Option<u64>,

  /// Set retry interval in seconds
  #[serde(default = "CommonConfig::default_retry_interval")]
  pub retry_interval: u64,
}

impl CommonConfig {
  pub fn default_retry_interval() -> u64 {
    5
  }
}

impl Default for CommonConfig {
  fn default() -> Self {
    Self {
      retry: None,
      retry_interval: Self::default_retry_interval(),
    }
  }
}
