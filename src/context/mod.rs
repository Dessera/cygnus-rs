pub mod common;
pub mod interface;
pub mod user;

use crate::{common::path::JludPath, config::Config, error::JludResult};

use self::{
  common::CommonContext, interface::InterfaceContext, user::UserContext,
};

/// 程序上下文对象
#[derive(Debug)]
pub struct Context {
  /// 用户上下文，用于用户信息API
  pub user: UserContext,
  /// 网络接口上下文，用于网络接口API
  pub interface: InterfaceContext,
  /// 通用上下文，用于通用API和运行时数据
  pub common: CommonContext,
  /// 用于存储程序路径信息
  pub path: JludPath,
}

impl Context {
  pub async fn create(config: Config, path: JludPath) -> JludResult<Self> {
    Ok(Self {
      user: UserContext::new(config.user),
      interface: InterfaceContext::create(config.interface).await?,
      common: CommonContext::new(config.common),
      path,
    })
  }
}
