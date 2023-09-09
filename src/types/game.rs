#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum Game {
  #[default]
  Unknown,
  Ets2,
  Ats
}

impl Game {
  pub fn from_u32(value: u32) -> Game {
    match value {
      1 => Self::Ets2,
      2 => Self::Ats,
      _ => Self::Unknown
    }
  }
}
