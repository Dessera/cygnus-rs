use pnet::datalink;
use serde::{Deserialize, Serialize};

/// Interface configurations
#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceConfig {
  /// Select the interface to use, if not provided, the first interface available will be used
  #[serde(default = "InterfaceConfig::default_interface")]
  pub interface: String,

  /// Set local UDP address
  #[serde(default = "InterfaceConfig::default_local_address")]
  pub local_address: String,

  /// Set local UDP port
  #[serde(default = "InterfaceConfig::default_local_port")]
  pub local_port: u16,

  /// Set remote UDP address
  #[serde(default = "InterfaceConfig::default_remote_address")]
  pub remote_address: String,

  /// Set remote UDP port
  #[serde(default = "InterfaceConfig::default_remote_port")]
  pub remote_port: u16,

  /// Set timeout in seconds
  #[serde(default = "InterfaceConfig::default_timeout")]
  pub timeout: u64,
}

impl InterfaceConfig {
  pub fn default_interface() -> String {
    match datalink::interfaces().iter().find(|interface| {
      interface.is_up() && !interface.is_loopback() && interface.is_running()
    }) {
      Some(interface) => interface.name.clone(),
      None => "".to_string(),
    }
  }

  pub fn default_local_address() -> String {
    "0.0.0.0".to_string()
  }

  pub fn default_local_port() -> u16 {
    0
  }

  pub fn default_remote_address() -> String {
    "10.100.61.3".to_string()
  }

  pub fn default_remote_port() -> u16 {
    61440
  }

  pub fn default_timeout() -> u64 {
    5
  }
}

impl Default for InterfaceConfig {
  fn default() -> Self {
    Self {
      interface: Self::default_interface(),
      local_address: Self::default_local_address(),
      local_port: Self::default_local_port(),
      remote_address: Self::default_remote_address(),
      remote_port: Self::default_remote_port(),
      timeout: Self::default_timeout(),
    }
  }
}
