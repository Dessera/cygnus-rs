pub mod challenge;
pub mod keep_alive;
pub mod login;
pub mod user;

use std::sync::Arc;

use clap::Parser;
use tokio::sync::Mutex;

use crate::{
  common::components::{InfiniteGuard, RetryGuard},
  component::Component,
  context::Context,
  error::JludResult,
};

use self::{
  challenge::Challenger, keep_alive::KeepAliveSender, login::LoginSender,
  user::UserInfoCollector,
};

#[derive(Parser, Debug)]
pub struct Auth;

impl Component for Auth {
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let (max_retries, interval) = {
      let ctx = context.lock().await;
      (ctx.common.config.retry, ctx.common.config.retry_interval)
    };
    RetryGuard::new(max_retries, interval, AuthImpl {})
      .run(context)
      .await
  }
}

#[derive(Debug)]
pub struct AuthImpl;

impl Component for AuthImpl {
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    UserInfoCollector::new().run(context.clone()).await?;

    Challenger::new().run(context.clone()).await?;
    LoginSender::new().run(context.clone()).await?;

    InfiniteGuard::new(20, KeepAliveSender::new())
      .run(context.clone())
      .await
  }
}
