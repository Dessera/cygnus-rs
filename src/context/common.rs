use crate::config::common::CommonConfig;

#[derive(Debug)]
pub struct CommonContext {
  pub config: CommonConfig,

  // runtime data
  pub salt: [u8; 4],
  pub client_ip: [u8; 4],
  pub md5a: [u8; 16],
  pub tail: [u8; 16],
  pub tail_2: [u8; 4],
  pub keep_alive_version: (u8, u8),
}

impl CommonContext {
  pub fn new(config: CommonConfig) -> Self {
    Self {
      config,
      salt: [0; 4],
      client_ip: [0; 4],
      md5a: [0; 16],
      tail: [0; 16],
      tail_2: [0; 4],
      keep_alive_version: (0, 0),
    }
  }
}
