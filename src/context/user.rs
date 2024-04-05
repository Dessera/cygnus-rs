use std::io::{stdin, stdout, Write};

use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use dotenv_codegen::dotenv;
use tokio::{
  fs::OpenOptions,
  io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};
use tracing::{info, warn};

use crate::{
  common::path::{app_config_dir, app_password_file},
  config::user::UserConfig,
  error::{JludError, JludResult},
};

#[derive(Debug)]
pub struct UserContext {
  pub config: UserConfig,

  pub username: Option<String>,
  pub password: Option<String>,
}

impl UserContext {
  pub fn new(config: UserConfig) -> Self {
    Self {
      config,
      username: None,
      password: None,
    }
  }

  pub fn read_user_info_from_prompt(&mut self) -> JludResult<()> {
    // prompt user for info
    stdout().write(b"Enter username: ")?;
    stdout().flush()?;
    let mut username = String::new();
    stdin().read_line(&mut username)?;
    self.username = Some(username.trim().to_string());
    // read password without echoing
    self.password = Some(rpassword::prompt_password("Enter password: ")?);
    Ok(())
  }

  #[tracing::instrument(skip_all)]
  pub async fn save_to_config(&self) -> JludResult<()> {
    // save user info to config file
    let base = match app_config_dir() {
      Some(base) => base,
      None => {
        warn!("Current platform does not support config file storage");
        return Err(JludError::UnsupportedPlatform);
      }
    };
    let path = match app_password_file() {
      Some(path) => path.to_string_lossy().to_string(),
      None => {
        warn!("Current platform does not support password file storage");
        return Err(JludError::UnsupportedPlatform);
      }
    };

    let key = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let cipher = Aes256Gcm::new(&key);

    let encrypted_password = match &self.password {
      Some(password) => cipher.encrypt(&nonce, password.as_bytes().as_ref())?,
      None => {
        warn!("No password to encrypt");
        return Ok(());
      }
    };
    let username = match &self.username {
      Some(username) => username,
      None => {
        warn!("No username to save");
        return Ok(());
      }
    };

    // check if base directory exists
    if !base.exists() {
      tokio::fs::create_dir_all(&base).await?;
    }

    let file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(&path)
      .await?;

    let mut writer = BufWriter::new(file);

    writer.write_all(key.as_ref()).await?;
    writer.write_all(nonce.as_ref()).await?;
    // write encrypted password size (fixed size u64)
    writer
      .write_all(&(encrypted_password.len() as u64).to_be_bytes())
      .await?;
    writer.write_all(&encrypted_password).await?;
    // write username
    writer
      .write_all(&(username.len() as u64).to_be_bytes())
      .await?;
    writer.write_all(username.as_bytes()).await?;

    writer.flush().await?;

    info!("Successfully saved user info to file: {}", path);

    Ok(())
  }

  #[tracing::instrument(skip_all)]
  pub async fn load_from_config(&mut self) -> JludResult<()> {
    // load user info from config file
    let path = match app_password_file() {
      Some(path) => path.to_string_lossy().to_string(),
      None => {
        warn!("Current platform does not support password file storage");
        return Err(JludError::UnsupportedPlatform);
      }
    };

    let file = OpenOptions::new().read(true).open(&path).await?;

    let mut reader = BufReader::new(file);

    let mut key = [0u8; 32];
    reader.read_exact(&mut key).await?;
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let mut nonce = [0u8; 12];
    reader.read_exact(&mut nonce).await?;
    let nonce = Nonce::from_slice(&nonce);

    let mut size_bytes = [0u8; 8];
    reader.read_exact(&mut size_bytes).await?;
    let size = u64::from_be_bytes(size_bytes);

    let mut encrypted_password = vec![0u8; size as usize];
    reader.read_exact(&mut encrypted_password).await?;

    let mut username_size_bytes = [0u8; 8];
    reader.read_exact(&mut username_size_bytes).await?;
    let username_size = u64::from_be_bytes(username_size_bytes);

    let mut username = vec![0u8; username_size as usize];
    reader.read_exact(&mut username).await?;

    let cipher = Aes256Gcm::new(&key);
    let password = cipher.decrypt(&nonce, encrypted_password.as_ref())?;

    self.username = Some(String::from_utf8(username)?);
    self.password = Some(String::from_utf8(password)?);

    info!("Successfully loaded user info from file: {}", path);

    Ok(())
  }
}
