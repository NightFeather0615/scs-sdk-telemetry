#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::{
  vector::{Vector32, Vector64},
  eular::{Eular32, Eular64}
};


#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Placement32 {
  pub position: Vector32,
  pub orientation: Eular32
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Placement64 {
  pub position: Vector64,
  pub orientation: Eular64
}
