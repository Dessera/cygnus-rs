#[derive(Default)]
pub struct DrContextData {
  // runtime data
  pub salt: [u8; 4],
  pub client_ip: [u8; 4],
  pub md5a: [u8; 16],
  pub tail: [u8; 16],
  pub tail_2: [u8; 4],
  pub keep_alive_version: (u8, u8),
}

pub enum AliveType {
  FIRST,
  SECOND,
  EXTRA,
}
