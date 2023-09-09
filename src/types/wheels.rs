#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::vector::Vector32;


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct WheelsConstants {
  pub count: u32,
  pub radius: Vec<f32>,
  pub simulated: Vec<bool>,
  pub powered: Vec<bool>,
  pub liftable: Vec<bool>,
  pub steerable: Vec<bool>,
  pub position: Vec<Vector32>,
}
