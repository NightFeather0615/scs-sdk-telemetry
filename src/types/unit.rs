#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::{
  enums::JobMarket,
  math::{
    Vector32,
    Vector64,
    Eular32,
    Eular64
  }
};


#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Placement32 {
  pub position: Vector32,
  pub orientation: Eular32
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Placement64 {
  pub position: Vector64,
  pub orientation: Eular64
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Substance {
  pub index: usize,
  pub value: String
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Navigation {
  pub navigation_distance: f32,
  pub navigation_time: f32,
  pub speed_limit: Movement
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Common {
  pub game_time: u32,
  pub next_rest_stop: i32,
  pub scale: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct WheelsConstants {
  pub count: u32,
  pub radius: Vec<f32>,
  pub simulated: Vec<bool>,
  pub powered: Vec<bool>,
  pub liftable: Vec<bool>,
  pub steerable: Vec<bool>,
  pub position: Vec<Vector32>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Version {
  pub major: u32,
  pub minor: u32
}

impl ToString for Version {
  fn to_string(&self) -> String {
    format!(
      "Version: {}.{}",
      self.major,
      self.minor
    )
  }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Job {
  pub delivery_time: u32,
  pub remaining_delivery_time: i64,
  pub cargo_loaded: bool,
  pub special_job: bool,
  pub market: JobMarket,
  pub planned_distance_km: u32,
  pub cargo: Cargo,
  pub city_destination_id: String,
  pub city_destination: String,
  pub company_destination_id: String,
  pub company_destination: String,
  pub city_source_id: String,
  pub city_source: String,
  pub company_source_id: String,
  pub company_source: String,
  pub income: u64
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Cargo {
  pub mass: f32,
  pub id: String,
  pub name: String,
  pub unit_count: u32,
  pub unit_mass: f32,
  pub cargo_damage: f32
}
