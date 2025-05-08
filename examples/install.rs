#[cfg(target_os = "linux")]
use lixploy::deploy;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  #[cfg(target_os = "linux")]
  deploy("ahq.AppImage", "/mnt/D60A42E70A42C3E9/GitHub/lixploy/resp", true).await.unwrap();
}