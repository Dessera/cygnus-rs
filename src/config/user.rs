use serde::{Deserialize, Serialize};

/// User configurations
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
  /// Control whether save user information
  #[serde(default)]
  pub save_user: bool,
}

impl Default for UserConfig {
  fn default() -> Self {
    Self { save_user: false }
  }
}
