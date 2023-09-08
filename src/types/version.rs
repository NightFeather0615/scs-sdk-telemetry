#[derive(Debug, Clone, Default)]
pub struct Version {
  pub major: u32,
  pub minor: u32
}

impl ToString for Version {
  fn to_string(&self) -> String {
    format!(
      "Version: {}.{}",
      self.major,
      self.minor
    )
  }
}
