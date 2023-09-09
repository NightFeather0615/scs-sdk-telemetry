#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::{vector::Vector, eular::Eular};


#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Placement<T> {
  pub position: Vector<T>,
  pub orientation: Eular<T>
}
