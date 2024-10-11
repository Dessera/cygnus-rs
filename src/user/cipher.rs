use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
use std::io::{BufReader, BufWriter, Read, Write};

use super::data::User;
use super::error::UserResult;

pub struct UserCipher;

impl UserCipher {
  pub fn encrypt<W: Write>(buffer: W, user: User) -> UserResult<()> {
    let key = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let cipher = Aes256Gcm::new(&key);

    let encrypted_password =
      cipher.encrypt(&nonce, user.password.as_bytes().as_ref())?;

    let mut writer = BufWriter::new(buffer);

    writer.write_all(&key)?;
    writer.write_all(&nonce)?;

    writer.write_all(&(encrypted_password.len() as u64).to_be_bytes())?;
    writer.write_all(&encrypted_password)?;

    writer.write_all(&(user.username.len() as u64).to_be_bytes())?;
    writer.write_all(user.username.as_bytes())?;

    writer.write_all(user.mac.as_ref())?;

    writer.flush()?;

    Ok(())
  }

  pub fn decrypt<R: Read>(buffer: R) -> UserResult<User> {
    let mut reader = BufReader::new(buffer);

    let mut key = [0u8; 32];
    reader.read_exact(&mut key)?;
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let mut nonce = [0u8; 12];
    reader.read_exact(&mut nonce)?;
    let nonce = Nonce::from_slice(&nonce);

    let mut size_bytes = [0u8; 8];
    reader.read_exact(&mut size_bytes)?;
    let size = u64::from_be_bytes(size_bytes);

    let mut encrypted_password = vec![0u8; size as usize];
    reader.read_exact(&mut encrypted_password)?;

    let mut username_size_bytes = [0u8; 8];
    reader.read_exact(&mut username_size_bytes)?;
    let username_size = u64::from_be_bytes(username_size_bytes);

    let mut username = vec![0u8; username_size as usize];
    reader.read_exact(&mut username)?;

    let mut mac = [0u8; 6];
    reader.read_exact(&mut mac)?;

    let cipher = Aes256Gcm::new(&key);
    let password = cipher.decrypt(&nonce, encrypted_password.as_ref())?;

    Ok(User::new(
      String::from_utf8(username)?,
      String::from_utf8(password)?,
      mac,
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encrypt_decrypt() {
    let user = User::new("user".to_string(), "password".to_string(), [0; 6]);
    let mut buffer = Vec::new();

    UserCipher::encrypt(&mut buffer, user.clone()).unwrap();
    let decrypted_user = UserCipher::decrypt(buffer.as_slice()).unwrap();

    assert_eq!(user, decrypted_user);
  }
}
