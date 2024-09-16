use tokio::fs;

pub async fn native_supported() -> Option<(bool, bool)> {
  let data = fs::read_to_string("/etc/os-release").await.ok()?;

  let is_deb = data.contains("debian");
  let is_rpm = ["suse", "fedora", "rhel", "centos"]
    .iter()
    .any(|x| data.contains(x));

  Some((is_deb, is_rpm))
}

pub async fn get_download<T: Iterator<Item = u8>>(data: T) -> Option<String> {
  let mut num = [0; 8];

  let mut total_deb = 0;

  let mut deb = Vec::with_capacity(64);
  let mut rpm = Vec::with_capacity(64);

  for (i, data) in data.enumerate() {
    if i < 8 {
      num[i] = data;

      if i == 7 {
        total_deb = u64::from_be_bytes(num);
      }
    } else if (i as u64) - 8 < total_deb {
      deb.push(data);
    } else {
      rpm.push(data);
    }
  }

  let deb = String::from_utf8_lossy(&deb).to_string();
  let rpm = String::from_utf8_lossy(&rpm).to_string();

  let (is_debian, is_rpm) = native_supported().await?;

  Some(if is_debian {
    deb
  } else if is_rpm {
    rpm
  } else {
    None?
  })
}

pub async fn generate<T: AsRef<str>, U: AsRef<str>>(deb: T, rpm: U) -> Vec<u8> {
  let mut data = vec![];

  let deb = deb.as_ref();
  let rpm = rpm.as_ref();
  data.append(&mut (deb.len() as u64).to_be_bytes().to_vec());
  data.append(&mut deb.as_bytes().to_vec());
  data.append(&mut rpm.as_bytes().to_vec());

  data
}
