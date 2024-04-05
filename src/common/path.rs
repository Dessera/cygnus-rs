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

pub fn app_password_file() -> Option<PathBuf> {
  app_config_dir().map(|mut p| {
    p.push(dotenv!("JLUD_APP_PASSWORD_FILE"));
    p
  })
}
