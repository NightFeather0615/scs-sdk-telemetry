#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::job_market::JobMarket;


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Job {
  pub delivery_time: u32,
  pub remaining_delivery_time: i32,
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
