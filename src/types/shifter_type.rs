#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


# [derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum ShifterType {
  #[default]
  Unknown,
  Arcade,
  Automatic,
  Manual,
  HShifter
}

impl ShifterType {
  pub fn from_str(value: &str) -> ShifterType {
    match value {
      "arcade" => Self::Arcade,
      "automatic" => Self::Automatic,
      "manual" => Self::Manual,
      "hshifter" => Self::HShifter,
      _ => Self::Unknown
    }
  }
}
