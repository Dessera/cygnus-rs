pub mod password;

#[derive(Debug)]
pub struct ContextConfig {
  pub password: password::PasswordStoreConfig,
}

#[derive(Debug)]
pub struct Context {
  password_store: password::PasswordStore,
}

impl Context {
  pub fn new(config: ContextConfig) -> Context {
    Context {
      password_store: password::PasswordStore::new(config.password),
    }
  }
}
