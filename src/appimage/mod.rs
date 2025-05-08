#[cfg(target_os = "linux")]
use std::io::ErrorKind;

#[cfg(target_os = "linux")]
use tokio::{fs, io::Error};

mod extract;
mod update;

#[cfg(target_os = "linux")]
pub async fn deploy(src: &str, to: &str, user: bool) -> Result<(), Error> {
  let _ = fs::remove_dir_all(to).await;
  fs::create_dir_all(to).await?;

  let app_img = format!("{to}/img.AppImage");
  fs::copy(src, &app_img).await?;
  if !extract::extract(to, &app_img).await? {
    return Err(Error::new(ErrorKind::Other, "Failed to extract AppImage"));
  }

  let from = format!("{to}/squashfs-root");
  let dir = format!("{to}/res");
  fs::rename(from, &dir).await?;

  update::update_appimg(&dir, user).await?;

  Ok(())
}