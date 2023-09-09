#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
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
