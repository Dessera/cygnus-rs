use std::sync::Arc;

use tokio::{
  sync::Mutex,
  time::{sleep, Duration},
};
use tracing::{error, info};

use crate::{component::Component, context::Context, error::JludResult};

#[derive(Debug)]
pub struct RetryGuard<C: Component + Send + Sync> {
  /// The maximum number of retries to attempt, if None, will retry infinitely
  max_retries: Option<i32>,
  timeout: u64,
  component: C,
}

impl<C: Component + Send + Sync> RetryGuard<C> {
  pub fn new(max_retries: Option<i32>, timeout: u64, component: C) -> Self {
    Self {
      max_retries,
      timeout,
      component,
    }
  }
}

impl<C: Component + Send + Sync> Component for RetryGuard<C> {
  #[tracing::instrument(skip_all)]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let mut retries = 0;

    info!("RetryGuard starting with timeout: {}", self.timeout);

    loop {
      match self.component.run(context.clone()).await {
        Ok(_) => {
          info!("RetryGuard succeeded after {} retries", retries);
          return Ok(());
        }
        Err(e) => {
          if let Some(max_retries) = self.max_retries {
            if retries >= max_retries {
              error!("RetryGuard failed after {} retries: {}", retries, e);
              return Err(e);
            }
          }
          error!("RetryGuard failed: {}, retrying", e);
          retries += 1;
          sleep(Duration::from_secs(self.timeout)).await;
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct InfiniteGuard<C: Component + Send + Sync> {
  timeout: u64,
  component: C,
}

impl<C: Component + Send + Sync> InfiniteGuard<C> {
  pub fn new(timeout: u64, component: C) -> Self {
    Self { timeout, component }
  }
}

impl<C: Component + Send + Sync> Component for InfiniteGuard<C> {
  #[tracing::instrument(skip_all)]
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    info!("InfiniteGuard running with timeout: {}", self.timeout);

    loop {
      self.component.run(context.clone()).await?;

      info!("InfiniteGuard sleeping for {} seconds", self.timeout);

      sleep(Duration::from_secs(self.timeout)).await;
    }
  }
}
