#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::{
  wheels::WheelsConstants,
  vector::Vector,
  placement::Placement
};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Trailer {
  pub wheels_constant: WheelsConstants,
  pub wheels: TrailerWheels,
  pub damage: TrailerDamage,
  pub acceleration: TrailerAcceleration,
  pub hook: Vector<f32>,
  pub attached: bool,
  pub body_type: String,
  pub brand: String,
  pub brand_id: String,
  pub cargo_accessory_id: String,
  pub chain_type: String,
  pub id: String,
  pub license_plate: String,
  pub license_plate_country: String,
  pub license_plate_country_id: String,
  pub name: String,
  pub position: Placement<f64>
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TrailerWheels {
  pub substance: Vec<u32>,
  pub suspension_deflection: Vec<f32>,
  pub velocity: Vec<f32>,
  pub steering: Vec<f32>,
  pub rotation: Vec<f32>,
  pub lift: Vec<f32>,
  pub lift_offset: Vec<f32>,
  pub on_ground: Vec<bool>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TrailerDamage {
  pub body: f32,
  pub cargo: f32,
  pub chassis: f32,
  pub wheels_avg: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TrailerAcceleration {
  pub linear_acceleration: Vector<f32>,
  pub linear_velocity: Vector<f32>,
  pub angular_acceleration: Vector<f32>,
  pub angular_velocity: Vector<f32>,
}
