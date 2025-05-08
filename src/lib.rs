#[cfg(target_os = "linux")]
pub mod appimage;

/// src: Source Folder
/// to: The place to Deploy
/// user: Deploy to ~/.local/share/applications
#[cfg(target_os = "linux")]
pub async fn deploy(src: &str, to: &str, user: bool) -> Result<(), tokio::io::Error> {
  appimage::deploy(src, to, user).await
}