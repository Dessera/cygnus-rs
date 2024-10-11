use super::error::UserResult;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
  pub username: String,
  pub password: String,
  pub mac: [u8; 6],
}

impl User {
  pub fn new(username: String, password: String, mac: [u8; 6]) -> Self {
    Self {
      username,
      password,
      mac,
    }
  }

  pub fn transform_mac(mac: &str) -> UserResult<[u8; 6]> {
    let mut mac_bytes = [0u8; 6];
    let mut i = 0;
    for byte in mac.split(':') {
      mac_bytes[i] = u8::from_str_radix(byte, 16)?;
      i += 1;
    }
    Ok(mac_bytes)
  }
}
