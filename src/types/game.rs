#[derive(Debug, Clone, Default)]
pub enum Game {
  #[default]
  Unknown,
  Ets2,
  Ats
}

impl Game {
  pub fn from_u32(value: u32) -> Game {
    match value {
      1 => Self::Ets2,
      2 => Self::Ats,
      _ => Self::Unknown
    }
  }
}
