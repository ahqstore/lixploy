use std::process::Stdio;

use tokio::process::Command;

pub async fn extract(dir: &str, path: &str) -> Result<bool, tokio::io::Error> {
  Ok(Command::new(path)
    .args(["--appimage-extract"])
    .current_dir(dir)
    .stdout(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?
    .wait()
    .await?
    .success())
}