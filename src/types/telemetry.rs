#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "json")]
use serde_json::{Result, Value};

use super::{
  version::Version,
  game::Game,
  common::Common,
  truck::Truck,
  job::Job,
  game_play_event::GamePlayEvents,
  control::Control,
  navigation::Navigation,
  placement::Placement64,
  special_events::SpecialEvents,
  substance::Substance,
  trailer::Trailer
};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TelemetryData {
  pub sdk_active: bool,
  pub paused: bool,
  pub timestamp: u64,
  pub simulation_timestamp: u64,
  pub render_timestamp: u64,
  pub multiplayer_time_offset: i64,
  pub dll_version: u32,
  pub game_version: Version,
  pub game: Game,
  pub telemetry_version: Version,
  pub common: Common,
  pub truck: Truck,
  pub job: Job,
  pub max_trailer_count: u32,
  pub game_play: GamePlayEvents,
  pub control: Control,
  pub navigation: Navigation,
  pub special_events: SpecialEvents,
  pub substances: Vec<Substance>,
  pub trailers: Vec<Trailer>
}

impl TelemetryData {
  /// Set `job.delivery_time` and `job.remaining_delivery_time`.
  pub fn set_delivery_time(self: &mut Self, delivery_time: u32) {
    self.job.delivery_time = delivery_time;
    if self.common.game_time > 0 && self.common.game_time < 4000000000 && delivery_time > 0 {
      self.job.remaining_delivery_time = (delivery_time - self.common.game_time) as i32;
    } else {
      self.job.remaining_delivery_time = 0;
    }
  }

  /// Set `truck.current.position` and `truck.positioning.truck_position`.
  pub fn set_truck_position(self: &mut Self, position: Placement64) {
    self.truck.current.position = position;
    self.truck.positioning.truck_position = position;
  }

  /// Serialize [types::TelemetryData] to [serde_json::Value].
  #[cfg(feature = "json")]
  pub fn to_json(self: &mut Self) -> Result<Value> {
    serde_json::to_value(self)
  }
}
