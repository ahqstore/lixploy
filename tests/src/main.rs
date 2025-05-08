#[tokio::main(flavor = "current_thread")]
async fn main() {
  #[cfg(target_os = "linux")]
  {
    use lixploy::{flatpak::flatpack_supported, native::{generate, native_supported}};
    use tokio::fs;

    native_supported().await;
    println!("{}", flatpack_supported().await);

    fs::write("./data", generate("deb", "rpm").await).await;
  }
}
