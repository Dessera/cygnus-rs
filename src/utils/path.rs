use std::path::PathBuf;

use dirs;
use dotenv_codegen::dotenv;

pub fn app_config_dir() -> Option<PathBuf> {
  dirs::config_dir().map(|mut p| {
    p.push(dotenv!("JLUD_APP_NAME"));
    p
  })
}

pub fn app_config_file() -> Option<PathBuf> {
  app_config_dir().map(|mut p| {
    p.push(dotenv!("JLUD_APP_CONFIG_FILE"));
    p
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_app_config_dir() {
    assert!(app_config_dir().is_some());
  }

  #[test]
  fn test_app_config_file() {
    assert!(app_config_file().is_some());
  }
}
