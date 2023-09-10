#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum JobMarket {
  #[default]
  NoValue,
  CargoMarket,
  QuickJob,
  FreightMarket,
  ExternalContracts,
  ExternalMarket
}

impl JobMarket {
  pub fn from_str(value: &str) -> JobMarket {
    match value {
      "cargo_market" => Self::CargoMarket,
      "quick_job" => Self::QuickJob,
      "freight_market" => Self::FreightMarket,
      "external_contracts" => Self::ExternalContracts,
      "external_market" => Self::ExternalMarket,
      _ => Self::NoValue
    }
  }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum Offence {
  #[default]
  NoValue,
  Crash,
  AvoidSleeping,
  WrongWay,
  SpeedingCamera,
  NoLights,
  RedSignal,
  AvoidWeighting,
  Speeding,
  IllegalTrailer,
  AvoidInspection,
  IllegalBorderCrossing,
  HardShoulderViolation,
  DamagedVehicleUsage,
  Generic
}

impl Offence {
  pub fn from_str(value: &str) -> Offence {
    match value {
      "crash" => Self::Crash,
      "avoid_sleeping" => Self::AvoidSleeping,
      "wrong_way" => Self::WrongWay,
      "speeding_camera" => Self::SpeedingCamera,
      "no_lights" => Self::NoLights,
      "red_signal" => Self::RedSignal,
      "avoid_weighting" => Self::AvoidWeighting,
      "speeding" => Self::Speeding,
      "illegal_trailer" => Self::IllegalTrailer,
      "avoid_Inspection" => Self::AvoidInspection,
      "illegal_Border_Crossing" => Self::IllegalBorderCrossing,
      "hard_Shoulder_Violation" => Self::HardShoulderViolation,
      "damaged_Vehicle_Usage" => Self::DamagedVehicleUsage,
      "generic" => Self::Generic,
      _ => Self::NoValue
    }
  }
}
