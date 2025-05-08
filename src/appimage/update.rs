#[cfg(target_os = "linux")]
use std::{env, io::Error};

#[cfg(target_os = "linux")]
use tokio::fs;

#[cfg(target_os = "linux")]
pub async fn update_appimg(dir: &str, user: bool) -> Result<(), Error> {
  let mut dir_dat = fs::read_dir(&dir).await?;

  let mut desktop_file = None;
  
  while let Some(dir) = dir_dat.next_entry().await? {
    let name = dir.file_name();  
    if let Ok(name) = name.into_string() {
      if name.ends_with(".desktop") {
        desktop_file = Some(name);
      }
    }
  }

  if let Some(x) = desktop_file {
    let desktop = format!("{dir}/{x}");

    let data = fs::read_to_string(&desktop).await?;

    let data: Vec<String> = data.lines()
      .map(|x| {
        if x.starts_with("Exec=") {
          return x.replacen("Exec=", &format!("Exec=env APPDIR=\"{}\" {}/", dir, dir), 1);
        } else if x.starts_with("Icon=") {
          let mut icon = x.replacen("Icon=", &format!("Icon={}/", dir), 1);

          icon.push_str(".png");

          return icon;
        }

        x.into()
      })
      .collect();
    let data = data.join("\n");

    fs::write(&desktop, data).await?;

    return install(&desktop, &x, user).await;
  }

  Err(Error::other("Unable to find .desktop file"))
}

#[cfg(target_os = "linux")]
async fn install(desktop: &str, x: &str, user: bool) -> Result<(), Error> {
  let dst = if user {
    let app = format!("{}/.local/share/applications", env::var("HOME").map_err(|_| Error::other("Could not find $HOME variable, is it set?"))?);

    fs::create_dir_all(&app).await?;

    format!("{}/{x}", app)
  } else {
    let local_app = "/usr/local/share/applications/";

    if let Err(_) = fs::read_dir(&local_app).await {
      format!("/usr/local/share/applications/{x}")
    } else {
      format!("/usr/share/applications/{x}")
    }
  };
  
  fs::symlink(desktop, dst).await
}