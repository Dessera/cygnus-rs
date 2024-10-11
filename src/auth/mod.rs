pub mod args;
pub mod context;
pub mod data;
pub mod error;

use std::fs::OpenOptions;

use args::AuthArgs;
use context::DrContext;
use data::AliveType;
use error::{AuthError, AuthResult};
use tracing::{error, info, warn};

use crate::user::cipher::UserCipher;

#[tracing::instrument(skip_all, name = "auth")]
pub fn auth_command_resolver(args: AuthArgs) -> AuthResult<()> {
  let fd = OpenOptions::new().read(true).open(&args.file)?;
  info!("Reading user data from file: {}", args.file);

  let user = UserCipher::decrypt(fd)?;
  info!("Target user: {}", user.username);

  let mut ctx = DrContext::try_new(user)?;
  info!("Starting authentication process");

  challenge(&mut ctx)?;
  login(&mut ctx)?;
  keep_alive(&mut ctx)?;

  Ok(())
}

#[tracing::instrument(skip_all)]
fn challenge(ctx: &mut DrContext) -> AuthResult<()> {
  let client = &ctx.client;

  info!("Starting challenge");

  let mut send_buf = [0; 20];
  let mut recv_buf = [0; 32];
  for try_times in 0..5 {
    info!("Challenge try: {}", try_times + 1);
    ctx.get_challenge_data(try_times, &mut send_buf);

    match client.send(&send_buf) {
      Ok(_) => {}
      Err(e) => {
        warn!("Failed to send challenge data: {}", e);
        continue;
      }
    }
    match client.recv(&mut recv_buf) {
      Ok(_) => {}
      Err(e) => {
        warn!("Failed to receive challenge data: {}", e);
        continue;
      }
    }
    if recv_buf[0] == 0x02 {
      ctx.data.salt.copy_from_slice(&recv_buf[4..8]);
      ctx.data.client_ip.copy_from_slice(&recv_buf[20..24]);
      info!("Challenge succeeded");
      return Ok(());
    }

    warn!("Challenge failed, retrying");
  }

  error!("Challenge max tries exceeded");
  Err(AuthError::ChallengeMaxTriesExceeded)
}

#[tracing::instrument(skip_all)]
fn login(ctx: &mut DrContext) -> AuthResult<()> {
  info!("Starting login,target user: {}", ctx.user.username);

  let mut send_buf = vec![0; 400];
  let mut recv_buf = [0; 48];

  ctx.get_login_data(&mut send_buf);
  ctx.client.send(&send_buf)?;

  ctx.client.recv(&mut recv_buf)?;

  if recv_buf[0] == 0x04 {
    info!("Login success");
    ctx.data.tail.copy_from_slice(&recv_buf[23..39]);
    return Ok(());
  } else if recv_buf[0] == 0x05 && recv_buf[4] == 0x0b {
    error!("Login failed: invalid mac");
    return Err(AuthError::InvalidMacAddress);
  } else if recv_buf[0] == 0x05 {
    error!("Login failed: invalid username or password");
    return Err(AuthError::InvalidUsernameOrPassword);
  }

  error!("Login failed: unknown error");
  Err(AuthError::Unknown)
}

#[tracing::instrument(skip_all)]
fn keep_alive(ctx: &mut DrContext) -> AuthResult<()> {
  info!("Starting keep alive");

  let mut send_buf_38 = [0; 38];
  let mut send_buf_40 = [0; 40];

  let mut recv_buf = [0; 32];
  let mut keep_40_count = 0u8;

  loop {
    info!("Sending keep alive data");

    ctx.get_keep_alive_data_38(&mut send_buf_38);
    ctx.client.send(&send_buf_38)?;
    ctx.client.recv(&mut recv_buf)?;
    ctx.data.keep_alive_version = (recv_buf[28], recv_buf[29]);

    if keep_40_count % 21 == 0 {
      ctx.get_keep_alive_data_40(
        AliveType::EXTRA,
        keep_40_count,
        &mut send_buf_40,
      );
      ctx.client.send(&send_buf_40)?;
      ctx.client.recv(&mut recv_buf)?;
      info!("Keep alive extra accepted");
    }

    ctx.get_keep_alive_data_40(
      AliveType::FIRST,
      keep_40_count,
      &mut send_buf_40,
    );
    ctx.client.send(&send_buf_40)?;
    ctx.client.recv(&mut recv_buf)?;
    ctx.data.tail_2.copy_from_slice(&recv_buf[16..20]);
    keep_40_count = keep_40_count.wrapping_add(1);
    info!("Keep alive first accepted");

    ctx.get_keep_alive_data_40(
      AliveType::SECOND,
      keep_40_count,
      &mut send_buf_40,
    );
    ctx.client.send(&send_buf_40)?;
    ctx.client.recv(&mut recv_buf)?;
    keep_40_count = keep_40_count.wrapping_add(1);
    info!("Keep alive second accepted");
    // sleep for 20 seconds
    std::thread::sleep(std::time::Duration::from_secs(20));
  }
}