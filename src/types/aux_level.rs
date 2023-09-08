#[derive(Debug, Clone, Default)]
pub enum AuxLevel {
  #[default]
  Off,
  Dimmed,
  Full
}

impl AuxLevel {
  pub fn from_u32(value: u32) -> AuxLevel {
    match value {
      1 => Self::Dimmed,
      2 => Self::Full,
      _ => Self::Off
    }
  }
}
