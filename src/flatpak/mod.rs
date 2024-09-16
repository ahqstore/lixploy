use std::process::Stdio;

use tokio::process::Command;

pub async fn flatpack_supported() -> bool {
  async {
    Some(
      Command::new("flatpak")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .ok()?
        .wait()
        .await
        .is_ok(),
    )
  }
  .await
  .unwrap_or(false)
}
