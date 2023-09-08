#[derive(Debug, Clone, Default)]
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
