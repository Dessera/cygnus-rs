use aes_gcm::{
  aead::{Aead, AeadCore, KeyInit, OsRng},
  Aes256Gcm, Key, Nonce,
};
use thiserror;
use tokio::io::{self as async_io, AsyncReadExt};
use tokio::{fs as async_fs, io::AsyncWriteExt};

#[derive(thiserror::Error, Debug)]
pub enum PasswordStoreError {
  #[error("Crypto error when encrypting password: {0}")]
  CryptoError(#[from] aes_gcm::Error),

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("UTF-8 error: {0}")]
  Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug)]
pub struct PasswordStoreConfig {
  pub store_path: String,
  pub save_password: bool,
}

#[derive(Debug)]
pub struct PasswordStore {
  config: PasswordStoreConfig,
}

impl PasswordStore {
  pub fn new(config: PasswordStoreConfig) -> PasswordStore {
    PasswordStore { config }
  }

  pub async fn load_password(&self) -> Result<String, PasswordStoreError> {
    let file = async_fs::File::open(&self.config.store_path).await?;

    let mut reader = async_io::BufReader::new(file);

    let mut nonce = Nonce::default();
    reader.read_exact(nonce.as_mut()).await?;

    let mut key = Key::<Aes256Gcm>::default();
    reader.read_exact(key.as_mut()).await?;

    let cipher = Aes256Gcm::new(&key);

    let mut encrypted_password = Vec::new();
    reader.read_to_end(&mut encrypted_password).await?;

    let password = cipher.decrypt(&nonce, encrypted_password.as_ref())?;

    Ok(String::from_utf8(password)?)
  }

  pub async fn save_password(
    &self,
    password: &str,
  ) -> Result<(), PasswordStoreError> {
    if !self.config.save_password {
      return Ok(());
    }

    let key = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(OsRng);

    let cipher = Aes256Gcm::new(&key);

    let encrypted_password =
      cipher.encrypt(&nonce, password.as_bytes().as_ref())?;

    let file = async_fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(&self.config.store_path)
      .await?;

    let mut writer = async_io::BufWriter::new(file);

    writer.write_all(nonce.as_ref()).await?;
    writer.write_all(key.as_slice()).await?;
    writer.write_all(encrypted_password.as_ref()).await?;

    writer.flush().await?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  #[ignore = "not a standalone test"]
  async fn test_save_password() {
    let config = PasswordStoreConfig {
      store_path: "jlud.passwd".to_string(),
      save_password: true,
    };

    let store = PasswordStore::new(config);

    assert!(store.save_password("password").await.is_ok());
  }

  #[tokio::test]
  #[ignore = "not a standalone test"]
  async fn test_load_password() {
    let config = PasswordStoreConfig {
      store_path: "jlud.passwd".to_string(),
      save_password: true,
    };

    let store = PasswordStore::new(config);

    assert_eq!(store.load_password().await.unwrap(), "password");
  }

  #[tokio::test]
  async fn test_save_and_load_password() {
    let config = PasswordStoreConfig {
      store_path: "jlud.passwd".to_string(),
      save_password: true,
    };

    let store = PasswordStore::new(config);

    assert!(store.save_password("password").await.is_ok());
    assert_eq!(store.load_password().await.unwrap(), "password");

    // rm the file
    async_fs::remove_file("jlud.passwd").await.unwrap();
  }
}
