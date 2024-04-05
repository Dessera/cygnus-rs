use tokio::net::UdpSocket;
use tokio::time::Duration;

use crate::{
  config::interface::InterfaceConfig,
  error::{JludError, JludResult},
};

#[derive(Debug)]
pub struct InterfaceContext {
  pub config: InterfaceConfig,
  socket: UdpSocket,
}

impl InterfaceContext {
  pub async fn create(config: InterfaceConfig) -> JludResult<Self> {
    let socket = UdpSocket::bind(format!(
      "{}:{}",
      config.local_address, config.local_port
    ))
    .await?;
    socket
      .connect(format!("{}:{}", config.remote_address, config.remote_port))
      .await?;
    Ok(Self { config, socket })
  }

  pub async fn send(&mut self, data: &[u8]) -> JludResult<()> {
    self.socket.send(data).await?;
    Ok(())
  }

  pub async fn recv(&mut self, data: &mut [u8]) -> JludResult<()> {
    self.socket.recv(data).await?;
    Ok(())
  }

  pub async fn recv_with_timeout(&mut self, data: &mut [u8]) -> JludResult<()> {
    let timeout = Duration::from_secs(5);
    let recv = tokio::time::timeout(timeout, self.socket.recv(data)).await;
    match recv {
      Ok(Ok(_)) => Ok(()),
      Ok(Err(e)) => Err(e.into()),
      Err(_) => Err(JludError::Timeout),
    }
  }
}
