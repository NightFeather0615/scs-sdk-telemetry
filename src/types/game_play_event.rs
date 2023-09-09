#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::offence::Offence;


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct GamePlayEvents {
  pub ferry_event: Transport,
  pub fined_event: Fined,
  pub job_cancelled: Cancelled,
  pub job_delivered: Delivered,
  pub tollgate_event: Tollgate,
  pub train_event: Transport,
  pub refuel_event: Refuel
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Cancelled {
  pub penalty: i64,
  pub finished: u32,
  pub started: u32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Delivered {
  pub auto_loaded: bool,
  pub auto_parked: bool,
  pub cargo_damage: f32,
  pub delivery_time: u32,
  pub distance_km: f32,
  pub earned_xp: i32,
  pub revenue: i64,
  pub finished: u32,
  pub started: u32,
}

impl Delivered {
  pub fn started_backup(self: &Self) -> u32 {
    self.finished - self.delivery_time
  }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Fined {
  pub amount: i64,
  pub offence: Offence 
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Tollgate {
  pub pay_amount: i64
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Transport {
  pub pay_amount: i64,
  pub source_id: String,
  pub source_name: String,
  pub target_id: String,
  pub target_name: String,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Refuel {
  pub amount: f32
}
