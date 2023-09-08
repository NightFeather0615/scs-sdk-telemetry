#[derive(Debug, Clone, Copy, Default)]
pub struct Eular<T> {
  pub heading: T,
  pub pitch: T,
  pub roll: T
}
