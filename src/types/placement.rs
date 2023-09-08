use super::{vector::Vector, eular::Eular};


#[derive(Debug, Clone, Copy, Default)]
pub struct Placement<T> {
  pub position: Vector<T>,
  pub orientation: Eular<T>
}
