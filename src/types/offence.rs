#[derive(Debug, Clone, Default)]
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
