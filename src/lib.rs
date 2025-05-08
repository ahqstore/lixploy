pub mod appimage;

/// src: Source Folder
/// to: The place to Deploy
/// user: Deploy to ~/.local/share/applications
pub async fn deploy(src: &str, to: &str, user: bool) -> Result<(), tokio::io::Error> {
  appimage::deploy(src, to, user).await
}