#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::movement::Movement;


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Navigation {
  pub navigation_distance: f32,
  pub navigation_time: f32,
  pub speed_limit: Movement
}
