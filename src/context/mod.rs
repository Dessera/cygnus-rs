pub mod common;
pub mod interface;
pub mod user;

use crate::{config::Config, error::JludResult};

use self::{
  common::CommonContext, interface::InterfaceContext, user::UserContext,
};

#[derive(Debug)]
pub struct Context {
  pub user: UserContext,
  pub interface: InterfaceContext,
  pub common: CommonContext,
}

impl Context {
  pub async fn create(config: Config) -> JludResult<Self> {
    Ok(Self {
      user: UserContext::new(config.user),
      interface: InterfaceContext::create(config.interface).await?,
      common: CommonContext::new(config.common),
    })
  }
}
