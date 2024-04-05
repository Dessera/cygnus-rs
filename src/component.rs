use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::{context::Context, error::JludError};

pub trait Component<E = JludError>
where
  E: Error,
{
  fn run(
    &mut self,
    context: Arc<Mutex<Context>>,
  ) -> impl std::future::Future<Output = Result<(), E>> + Send
  where
    Self: Send + Sync,
    E: Send + Sync;
}
