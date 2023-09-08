# [derive(Debug, Clone, Default)]
pub enum ShifterType {
  #[default]
  Unknown,
  Arcade,
  Automatic,
  Manual,
  HShifter
}

impl ShifterType {
  pub fn from_str(value: &str) -> ShifterType {
    match value {
      "arcade" => Self::Arcade,
      "automatic" => Self::Automatic,
      "manual" => Self::Manual,
      "hshifter" => Self::HShifter,
      _ => Self::Unknown
    }
  }
}
