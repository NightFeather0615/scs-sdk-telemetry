#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Vector32 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Vector64 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Eular32 {
  pub heading: f32,
  pub pitch: f32,
  pub roll: f32,
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Eular64 {
  pub heading: f64,
  pub pitch: f64,
  pub roll: f64,
}
