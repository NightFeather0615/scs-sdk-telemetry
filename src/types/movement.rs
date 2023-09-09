#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Movement {
  pub value: f32
}

impl Movement {
  pub fn kph(self: &Self) -> f32 {
    self.value * 3.6
  }

  pub fn mph(self: &Self) -> f32 {
    self.value * 2.25
  }
}
