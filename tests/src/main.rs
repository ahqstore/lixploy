use lixploy::{flatpak::flatpack_supported, native::{generate, native_supported}};
use tokio::fs;

#[tokio::main]
async fn main() {
  native_supported().await;
  println!("{}", flatpack_supported().await);

  fs::write("./data", generate("deb", "rpm").await).await;
}
