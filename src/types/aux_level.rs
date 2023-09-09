#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum AuxLevel {
  #[default]
  Off,
  Dimmed,
  Full
}

impl AuxLevel {
  pub fn from_u32(value: u32) -> AuxLevel {
    match value {
      1 => Self::Dimmed,
      2 => Self::Full,
      _ => Self::Off
    }
  }
}
