use std::path::PathBuf;

use directories::ProjectDirs;
use dotenv_codegen::dotenv;

#[derive(Debug)]
pub struct JludPath {
  /// Path to the configuration file
  pub config: PathBuf,
  /// Path to the user data file
  pub user: PathBuf,
}

impl JludPath {
  pub fn new(config: PathBuf, user: PathBuf) -> Self {
    Self { config, user }
  }
}

impl From<ProjectDirs> for JludPath {
  fn from(dirs: ProjectDirs) -> Self {
    Self {
      config: dirs
        .config_dir()
        .to_path_buf()
        .join(dotenv!("JLUD_APP_CONFIG_FILE")),
      user: dirs
        .cache_dir()
        .to_path_buf()
        .join(dotenv!("JLUD_APP_PASSWORD_FILE")),
    }
  }
}
