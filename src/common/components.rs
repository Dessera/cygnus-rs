use std::sync::Arc;

use tokio::{
  sync::Mutex,
  time::{sleep, Duration},
};

use crate::{component::Component, context::Context, error::JludResult};

#[derive(Debug)]
pub struct RetryGuard<C: Component + Send + Sync> {
  /// The maximum number of retries to attempt, if None, will retry infinitely
  max_retries: Option<u32>,
  timeout: u64,
  component: C,
}

impl<C: Component + Send + Sync> Component for RetryGuard<C> {
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    let mut retries = 0;
    loop {
      match self.component.run(context.clone()).await {
        Ok(_) => return Ok(()),
        Err(e) => {
          if let Some(max_retries) = self.max_retries {
            if retries >= max_retries {
              return Err(e);
            }
          }
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
  async fn run(&mut self, context: Arc<Mutex<Context>>) -> JludResult<()> {
    loop {
      self.component.run(context.clone()).await?;
      sleep(Duration::from_secs(self.timeout)).await;
    }
  }
}
