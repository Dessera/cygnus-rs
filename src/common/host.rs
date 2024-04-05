pub fn get_host_name() -> String {
  match hostname::get() {
    Ok(host) => host.to_string_lossy().to_string(),
    Err(_) => "unknown".to_string(),
  }
}
