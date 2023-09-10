#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};

use super::{
  math::Vector32,
  enums::{
    AuxLevel,
    ShifterType
  },
  unit::{
    WheelsConstants,
    Placement32,
    Placement64,
    Movement
  },
};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Truck {
  pub constants: Constants,
  pub current: Current,
  pub positioning: Position
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Constants {
  pub motor: MotorConstants,
  pub capacity: Capacity,
  pub warning_factor: WarningFactor,
  pub wheels: WheelsConstants,
  pub brand_id: String,
  pub brand: String,
  pub id: String,
  pub name: String,
  pub license_plate: String,
  pub license_plate_country_id: String,
  pub license_plate_country: String
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct MotorConstants {
  pub forward_gear_count: u32,
  pub reverse_gear_count: u32,
  pub retarder_step_count: u32,
  pub selector_count: u32,
  pub slot_gear: Vec<i32>,
  pub slot_handle_position: Vec<u32>,
  pub slot_selectors: Vec<u32>,
  pub engine_rpm_max: f32,
  pub differential_ration: f32,
  pub gear_ratios_forward: Vec<f32>,
  pub gear_ratios_reverse: Vec<f32>,
  pub shifter_type: ShifterType
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Capacity {
  pub fuel: f32,
  pub ad_blue: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct WarningFactor {
  pub fuel: f32,
  pub ad_blue: f32,
  pub air_pressure: f32,
  pub air_pressure_emergency: f32,
  pub oil_pressure: f32,
  pub water_temperature: f32,
  pub battery_voltage: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Current {
  pub electric_enabled: bool,
  pub engine_enabled: bool,
  pub differential_lock: bool,
  pub lift_axle: bool,
  pub lift_axle_indicator: bool,
  pub trailer_lift_axle: bool,
  pub trailer_lift_axle_indicator: bool,
  pub motor: Motor,
  pub dashboard: Dashboard,
  pub lights: Lights,
  pub wheels: TruckWheels,
  pub damage: TruckDamage,
  pub acceleration: TruckAcceleration,
  pub position: Placement64
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Position {
  pub cabin: Vector32,
  pub head: Vector32,
  pub hook: Vector32,
  pub cabin_offset: Placement32,
  pub head_offset: Placement32,
  pub truck_position: Placement64,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Motor {
  pub gear: Gear,
  pub brake: Brakes
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Gear {
  pub h_shifter_slot: u32,
  pub selected: i32,
  pub h_shifter_selector: Vec<bool>
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Brakes {
  pub retarder_level: u32,
  pub air_pressure: f32,
  pub temperature: f32,
  pub parking_brake: bool,
  pub motor_brake: bool
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Dashboard {
  pub fuel: Fuel,
  pub warning: Warnings,
  pub gear_dashboards: i32,
  pub speed: Movement,
  pub cruise_control_speed: Movement,
  pub ad_blue: f32,
  pub oil_pressure: f32,
  pub oil_temperature: f32,
  pub water_temperature: f32,
  pub battery_voltage: f32,
  pub rpm: f32,
  pub odometer: f32,
  pub wipers: bool,
  pub cruise_control: bool
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Fuel {
  pub amount: f32,
  pub average_consumption: f32,
  pub range: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Warnings {
  pub air_pressure: bool,
  pub air_pressure_emergency: bool,
  pub fuel_warning: bool,
  pub ad_blue: bool,
  pub oil_pressure: bool,
  pub water_temperature: bool,
  pub battery_voltage: bool
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct Lights {
  pub aux_front: AuxLevel,
  pub aux_roof: AuxLevel,
  pub dashboard_backlight: f32,
  pub blinker_left_active: bool,
  pub blinker_left_on: bool,
  pub blinker_right_active: bool,
  pub blinker_right_on: bool,
  pub parking: bool,
  pub beam_low: bool,
  pub beam_high: bool,
  pub beacon: bool,
  pub brake: bool,
  pub reverse: bool,
  pub hazard_warning_lights: bool
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TruckWheels {
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
pub struct TruckDamage {
  pub engine: f32,
  pub transmission: f32,
  pub cabin: f32,
  pub chassis: f32,
  pub wheels_avg: f32
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub struct TruckAcceleration {
  pub linear_acceleration: Vector32,
  pub linear_velocity: Vector32,
  pub angular_acceleration: Vector32,
  pub angular_velocity: Vector32,
  pub cabin_angular_acceleration: Vector32,
  pub cabin_angular_velocity: Vector32,
}
