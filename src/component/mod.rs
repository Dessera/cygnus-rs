pub mod connection;
pub mod runtime;

use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::context::Context;

pub trait Component<E>
where
  E: Error,
{
  fn run(
    &self,
    context: Arc<Mutex<Context>>,
  ) -> impl std::future::Future<Output = Result<(), E>> + Send
  where
    Self: Send + Sync,
    E: Send + Sync;
}
