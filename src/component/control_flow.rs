use std::{error::Error, fmt::Debug, sync::Arc, time::Duration};

use tokio::{select, sync::Mutex, time::sleep};
use tracing::{event, Level};

use crate::context::Context;

use super::Component;

#[derive(Debug)]
pub struct RetryGuard<C> {
  pub max_retries: u32,
  pub delay: Duration,
  pub component: C,
}

impl<C> RetryGuard<C> {
  pub fn new(max_retries: u32, delay: Duration, component: C) -> Self {
    RetryGuard {
      max_retries,
      delay,
      component,
    }
  }
}

impl<E, C> Component<E> for RetryGuard<C>
where
  E: Error + Send + Sync,
  C: Component<E> + Send + Sync + Debug,
{
  #[tracing::instrument(skip_all, name = "retry_guard")]
  async fn run(&self, context: Arc<Mutex<Context>>) -> Result<(), E> {
    let mut retries = 0;
    loop {
      match self.component.run(context.clone()).await {
        Ok(_) => return Ok(()),
        Err(err) => {
          if retries >= self.max_retries {
            event!(Level::ERROR, "Max retries exceeded: {:?}", err);
            return Err(err);
          }
          event!(Level::WARN, "Error occurred: {:?}, retrying", err);
          retries += 1;
          sleep(self.delay).await;
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct Parallel<CA, CB> {
  pub a: CA,
  pub b: CB,
}

impl<CA, CB> Parallel<CA, CB> {
  pub fn new(a: CA, b: CB) -> Self {
    Parallel { a, b }
  }
}

impl<E, CA, CB> Component<E> for Parallel<CA, CB>
where
  E: Error + Send + Sync,
  CA: Component<E> + Send + Sync,
  CB: Component<E> + Send + Sync,
{
  async fn run(&self, context: Arc<Mutex<Context>>) -> Result<(), E> {
    select! {
      r = self.a.run(context.clone()) => r,
      r = self.b.run(context) => r,
    }
  }
}

/// A component that runs infinitely until an error occurs.
#[derive(Debug)]
pub struct Infinite<C> {
  pub component: C,
  pub delay: Duration,
}

impl<C> Infinite<C> {
  pub fn new(component: C, delay: Duration) -> Self {
    Infinite { component, delay }
  }
}

impl<E, C> Component<E> for Infinite<C>
where
  E: Error + Send + Sync,
  C: Component<E> + Send + Sync,
{
  async fn run(&self, context: Arc<Mutex<Context>>) -> Result<(), E> {
    loop {
      match self.component.run(context.clone()).await {
        Ok(_) => sleep(self.delay).await,
        Err(err) => return Err(err),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use tokio::sync::Mutex;

  use crate::context::Context;

  use super::*;

  use std::io::Error as IoError;

  #[derive(Debug)]
  struct TestComponent;

  impl Component<IoError> for TestComponent {
    async fn run(&self, _context: Arc<Mutex<Context>>) -> Result<(), IoError> {
      Ok(())
    }
  }

  #[tokio::test]
  async fn test_retry_guard() {
    let component = RetryGuard::new(3, Duration::from_secs(1), TestComponent);
    let context = Arc::new(Mutex::new(Context::default()));
    assert!(component.run(context).await.is_ok());
  }

  #[tokio::test]
  async fn test_parallel() {
    let component = Parallel::new(TestComponent, TestComponent);
    let context = Arc::new(Mutex::new(Context::default()));
    assert!(component.run(context).await.is_ok());
  }

  #[tokio::test]
  #[ignore = "infinite test"]
  async fn test_infinite() {
    let component = Infinite::new(TestComponent, Duration::from_secs(1));
    let context = Arc::new(Mutex::new(Context::default()));
    assert!(component.run(context).await.is_ok());
  }
}
