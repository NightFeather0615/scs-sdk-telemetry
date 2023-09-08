use std::{
  ffi::c_void,
  mem::size_of,
  str,
  slice
};

use crate::types::{
  telemetry::TelemetryData,
  game::Game,
  aux_level::AuxLevel,
  vector::Vector,
  placement::Placement,
  eular::Eular,
  shifter_type::ShifterType,
  job_market::JobMarket,
  offence::Offence,
  substance::Substance, trailer::Trailer
};


const STRING_SIZE: usize = 64;
const SUBSTANCES: usize = 25;
const WHEEL_SIZE: usize = 16;
const OFFSET_AREAS: [isize; 14] = [
  0, 40, 500, 700, 1500, 1640, 2000, 2200, 2300, 4000, 4200, 4300, 4400, 6000
];


pub struct SdkConverter {
  data_pointer: *const c_void,
  offset: isize,
  offset_area: usize
}

impl SdkConverter {
  pub fn new(data_pointer: *const c_void) -> Self {
    SdkConverter {
      data_pointer,
      offset: 0,
      offset_area: 0
    }
  }

  pub fn convert(self: &mut Self) -> TelemetryData {
    let mut telemetry_data: TelemetryData = TelemetryData::default();

    self.offset = 0;


    // 1st Zone

    telemetry_data.sdk_active = self.get_value();
    self.pad_offset(3);

    telemetry_data.paused = self.get_value();
    self.pad_offset(3);

    telemetry_data.timestamp = self.get_value();
    telemetry_data.simulation_timestamp = self.get_value();
    telemetry_data.render_timestamp = self.get_value();
    telemetry_data.multiplayer_time_offset = self.get_value();

    self.next_offset_area();


    // 2nd Zone

    telemetry_data.dll_version = self.get_value();

    telemetry_data.game_version.major = self.get_value();
    telemetry_data.game_version.minor = self.get_value();

    telemetry_data.game = Game::from_u32(self.get_value());

    telemetry_data.telemetry_version.major = self.get_value();
    telemetry_data.telemetry_version.minor = self.get_value();

    telemetry_data.common.game_time = self.get_value();

    telemetry_data.truck.constants.motor.forward_gear_count = self.get_value();
    telemetry_data.truck.constants.motor.reverse_gear_count = self.get_value();
    telemetry_data.truck.constants.motor.retarder_step_count = self.get_value();
    telemetry_data.truck.constants.wheels.count = self.get_value();
    telemetry_data.truck.constants.motor.selector_count = self.get_value();

    telemetry_data.set_delivery_time(self.get_value());

    telemetry_data.max_trailer_count = self.get_value();

    telemetry_data.job.cargo.unit_count = self.get_value();
    telemetry_data.job.planned_distance_km = self.get_value();

    telemetry_data.truck.current.motor.gear.h_shifter_slot = self.get_value();
    telemetry_data.truck.current.motor.brake.retarder_level = self.get_value();
    telemetry_data.truck.current.lights.aux_front = AuxLevel::from_u32(self.get_value());
    telemetry_data.truck.current.lights.aux_roof = AuxLevel::from_u32(self.get_value());
    telemetry_data.truck.current.wheels.substance = self.get_value_vec(WHEEL_SIZE);

    telemetry_data.truck.constants.motor.slot_handle_position = self.get_value_vec(32);
    telemetry_data.truck.constants.motor.slot_selectors = self.get_value_vec(32);

    self.next_offset_area();


    // 3rd Zone

    telemetry_data.common.next_rest_stop = self.get_value();

    telemetry_data.truck.current.motor.gear.selected = self.get_value();
    telemetry_data.truck.current.dashboard.gear_dashboards = self.get_value();
    telemetry_data.truck.constants.motor.slot_gear = self.get_value_vec(32);

    telemetry_data.game_play.job_delivered.earned_xp = self.get_value();

    self.next_offset_area();


    // 4th Zone

    telemetry_data.common.scale = self.get_value();

    telemetry_data.truck.constants.capacity.fuel = self.get_value();
    telemetry_data.truck.constants.warning_factor.fuel = self.get_value();
    telemetry_data.truck.constants.capacity.ad_blue = self.get_value();
    telemetry_data.truck.constants.warning_factor.ad_blue = self.get_value();
    telemetry_data.truck.constants.warning_factor.air_pressure = self.get_value();
    telemetry_data.truck.constants.warning_factor.air_pressure_emergency = self.get_value();
    telemetry_data.truck.constants.warning_factor.oil_pressure = self.get_value();
    telemetry_data.truck.constants.warning_factor.water_temperature = self.get_value();
    telemetry_data.truck.constants.warning_factor.battery_voltage = self.get_value();
    telemetry_data.truck.constants.motor.engine_rpm_max = self.get_value();
    telemetry_data.truck.constants.motor.differential_ration = self.get_value();

    telemetry_data.job.cargo.mass = self.get_value();

    telemetry_data.truck.constants.wheels.radius = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.constants.motor.gear_ratios_forward = self.get_value_vec(24);
    telemetry_data.truck.constants.motor.gear_ratios_reverse = self.get_value_vec(8);

    telemetry_data.job.cargo.unit_mass = self.get_value();
    
    telemetry_data.truck.current.dashboard.speed.value = self.get_value();
    telemetry_data.truck.current.dashboard.rpm = self.get_value();

    telemetry_data.control.raw_input.steering = self.get_value();
    telemetry_data.control.raw_input.throttle = self.get_value();
    telemetry_data.control.raw_input.brake = self.get_value();
    telemetry_data.control.raw_input.clutch = self.get_value();
    telemetry_data.control.game_input.steering = self.get_value();
    telemetry_data.control.game_input.throttle = self.get_value();
    telemetry_data.control.game_input.brake = self.get_value();
    telemetry_data.control.game_input.clutch = self.get_value();

    telemetry_data.truck.current.dashboard.cruise_control_speed.value = self.get_value();
    telemetry_data.truck.current.motor.brake.air_pressure = self.get_value();
    telemetry_data.truck.current.motor.brake.temperature = self.get_value();
    telemetry_data.truck.current.dashboard.fuel.amount = self.get_value();
    telemetry_data.truck.current.dashboard.fuel.average_consumption = self.get_value();
    telemetry_data.truck.current.dashboard.fuel.range = self.get_value();
    telemetry_data.truck.current.dashboard.ad_blue = self.get_value();
    telemetry_data.truck.current.dashboard.oil_pressure = self.get_value();
    telemetry_data.truck.current.dashboard.oil_temperature = self.get_value();
    telemetry_data.truck.current.dashboard.water_temperature = self.get_value();
    telemetry_data.truck.current.dashboard.battery_voltage = self.get_value();
    telemetry_data.truck.current.lights.dashboard_backlight = self.get_value();
    telemetry_data.truck.current.damage.engine = self.get_value();
    telemetry_data.truck.current.damage.transmission = self.get_value();
    telemetry_data.truck.current.damage.cabin = self.get_value();
    telemetry_data.truck.current.damage.chassis = self.get_value();
    telemetry_data.truck.current.damage.wheels_avg = self.get_value();
    telemetry_data.truck.current.dashboard.odometer = self.get_value();

    telemetry_data.navigation.navigation_distance = self.get_value();
    telemetry_data.navigation.navigation_time = self.get_value();
    telemetry_data.navigation.speed_limit = self.get_value();

    telemetry_data.truck.current.wheels.suspension_deflection = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.wheels.velocity = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.wheels.steering = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.wheels.rotation = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.wheels.lift = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.wheels.lift_offset = self.get_value_vec(WHEEL_SIZE);

    telemetry_data.game_play.job_delivered.cargo_damage = self.get_value();
    telemetry_data.game_play.job_delivered.distance_km = self.get_value();
    telemetry_data.game_play.refuel_event.amount = self.get_value();

    telemetry_data.job.cargo.cargo_damage = self.get_value();

    self.next_offset_area();


    // 5th Zone

    telemetry_data.truck.constants.wheels.steerable = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.constants.wheels.simulated = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.constants.wheels.powered = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.constants.wheels.liftable = self.get_value_vec(WHEEL_SIZE);

    telemetry_data.job.cargo_loaded = self.get_value();
    telemetry_data.job.special_job = self.get_value();

    telemetry_data.truck.current.motor.brake.parking_brake = self.get_value();
    telemetry_data.truck.current.motor.brake.motor_brake = self.get_value();
    telemetry_data.truck.current.dashboard.warning.air_pressure = self.get_value();
    telemetry_data.truck.current.dashboard.warning.air_pressure_emergency = self.get_value();
    telemetry_data.truck.current.dashboard.warning.fuel_warning = self.get_value();
    telemetry_data.truck.current.dashboard.warning.ad_blue = self.get_value();
    telemetry_data.truck.current.dashboard.warning.oil_pressure = self.get_value();
    telemetry_data.truck.current.dashboard.warning.water_temperature = self.get_value();
    telemetry_data.truck.current.dashboard.warning.battery_voltage = self.get_value();
    telemetry_data.truck.current.electric_enabled = self.get_value();
    telemetry_data.truck.current.engine_enabled = self.get_value();
    telemetry_data.truck.current.dashboard.wipers = self.get_value();
    telemetry_data.truck.current.lights.blinker_left_active = self.get_value();
    telemetry_data.truck.current.lights.blinker_right_active = self.get_value();
    telemetry_data.truck.current.lights.blinker_left_on = self.get_value();
    telemetry_data.truck.current.lights.blinker_right_on = self.get_value();
    telemetry_data.truck.current.lights.parking = self.get_value();
    telemetry_data.truck.current.lights.beam_low = self.get_value();
    telemetry_data.truck.current.lights.beam_high = self.get_value();
    telemetry_data.truck.current.lights.beacon = self.get_value();
    telemetry_data.truck.current.lights.brake = self.get_value();
    telemetry_data.truck.current.lights.reverse = self.get_value();
    telemetry_data.truck.current.lights.hazard_warning_lights = self.get_value();
    telemetry_data.truck.current.dashboard.cruise_control = self.get_value();
    telemetry_data.truck.current.wheels.on_ground = self.get_value_vec(WHEEL_SIZE);
    telemetry_data.truck.current.motor.gear.h_shifter_selector = self.get_value_vec(2);
    telemetry_data.truck.current.differential_lock = self.get_value();
    telemetry_data.truck.current.lift_axle = self.get_value();
    telemetry_data.truck.current.lift_axle_indicator = self.get_value();
    telemetry_data.truck.current.trailer_lift_axle = self.get_value();
    telemetry_data.truck.current.trailer_lift_axle_indicator = self.get_value();

    telemetry_data.game_play.job_delivered.auto_parked = self.get_value();
    telemetry_data.game_play.job_delivered.auto_loaded = self.get_value();

    self.next_offset_area();


    // 6th Zone

    telemetry_data.truck.positioning.cabin = self.get_vector();
    telemetry_data.truck.positioning.head = self.get_vector();
    telemetry_data.truck.positioning.hook = self.get_vector();
    telemetry_data.truck.constants.wheels.position = {
      let mut temp_pos: Vec<Vector<f32>> = vec![Vector::default(); WHEEL_SIZE];

      temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.x = self.get_value());
      temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.y = self.get_value());
      temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.z = self.get_value());

      temp_pos
    };
    telemetry_data.truck.current.acceleration.linear_velocity = self.get_vector();
    telemetry_data.truck.current.acceleration.angular_velocity = self.get_vector();
    telemetry_data.truck.current.acceleration.linear_acceleration = self.get_vector();
    telemetry_data.truck.current.acceleration.angular_acceleration = self.get_vector();
    telemetry_data.truck.current.acceleration.cabin_angular_velocity = self.get_vector();
    telemetry_data.truck.current.acceleration.cabin_angular_acceleration = self.get_vector();

    self.next_offset_area();


    // 7th Zone

    telemetry_data.truck.positioning.cabin_offset = self.get_placement();
    telemetry_data.truck.positioning.head_offset = self.get_placement();

    self.next_offset_area();


    // 8th Zone

    telemetry_data.set_truck_position(self.get_placement());

    self.next_offset_area();


    // 9th Zone

    telemetry_data.truck.constants.brand_id = self.get_string(None);
    telemetry_data.truck.constants.brand = self.get_string(None);
    telemetry_data.truck.constants.id = self.get_string(None);
    telemetry_data.truck.constants.name = self.get_string(None);

    telemetry_data.job.cargo.id = self.get_string(None);
    telemetry_data.job.cargo.name = self.get_string(None);
    telemetry_data.job.city_destination_id = self.get_string(None);
    telemetry_data.job.city_destination = self.get_string(None);
    telemetry_data.job.company_destination_id = self.get_string(None);
    telemetry_data.job.company_destination = self.get_string(None);
    telemetry_data.job.city_source_id = self.get_string(None);
    telemetry_data.job.city_source = self.get_string(None);
    telemetry_data.job.company_source_id = self.get_string(None);
    telemetry_data.job.company_source = self.get_string(None);

    telemetry_data.truck.constants.motor.shifter_type = ShifterType::from_str(&self.get_string(Some(16)));
    telemetry_data.truck.constants.license_plate = self.get_string(None);
    telemetry_data.truck.constants.license_plate_country_id = self.get_string(None);
    telemetry_data.truck.constants.license_plate_country = self.get_string(None);

    telemetry_data.job.market = JobMarket::from_str(&self.get_string(Some(32)));

    telemetry_data.game_play.fined_event.offence = Offence::from_str(&self.get_string(Some(32)));

    telemetry_data.game_play.ferry_event.source_name = self.get_string(None);
    telemetry_data.game_play.ferry_event.target_name = self.get_string(None);
    telemetry_data.game_play.ferry_event.source_id = self.get_string(None);
    telemetry_data.game_play.ferry_event.target_id = self.get_string(None);
    telemetry_data.game_play.train_event.source_name = self.get_string(None);
    telemetry_data.game_play.train_event.target_name = self.get_string(None);
    telemetry_data.game_play.train_event.source_id = self.get_string(None);
    telemetry_data.game_play.train_event.target_id = self.get_string(None);

    self.next_offset_area();


    // 10th Zone

    telemetry_data.job.income = self.get_value();

    self.next_offset_area();


    // 11th Zone

    telemetry_data.game_play.job_cancelled.penalty = self.get_value();
    telemetry_data.game_play.job_delivered.revenue = self.get_value();
    telemetry_data.game_play.fined_event.amount = self.get_value();
    telemetry_data.game_play.tollgate_event.pay_amount = self.get_value();
    telemetry_data.game_play.ferry_event.pay_amount = self.get_value();
    telemetry_data.game_play.train_event.pay_amount = self.get_value();

    self.next_offset_area();


    // 12th Zone

    telemetry_data.special_events.on_job = self.get_value();
    telemetry_data.special_events.job_finished = self.get_value();
    telemetry_data.special_events.job_cancelled = self.get_value();
    telemetry_data.special_events.job_delivered = self.get_value();
    telemetry_data.special_events.fined = self.get_value();
    telemetry_data.special_events.tollgate = self.get_value();
    telemetry_data.special_events.ferry = self.get_value();
    telemetry_data.special_events.train = self.get_value();
    telemetry_data.special_events.refuel = self.get_value();
    telemetry_data.special_events.refuel_payed = self.get_value();

    self.next_offset_area();


    // 13th Zone

    telemetry_data.substances = {
      let mut temp_sub: Vec<Substance> = vec![Substance::default(); SUBSTANCES];

      temp_sub.iter_mut().enumerate().for_each(
        |(i, v): (usize, &mut Substance)| {
          v.index = i;
          v.value = self.get_string(None);
        }
      );
      temp_sub.retain(|x: &Substance| x.value.len() > 0);

      temp_sub
    };

    self.next_offset_area();


    // 14th Zone

    telemetry_data.trailers = {
      let mut temp_trailers: Vec<Trailer> = vec![Trailer::default(); 10];

      temp_trailers.iter_mut().for_each(
        |t: &mut Trailer| {
          t.wheels_constant.steerable = self.get_value_vec(WHEEL_SIZE);
          t.wheels_constant.simulated = self.get_value_vec(WHEEL_SIZE);
          t.wheels_constant.powered = self.get_value_vec(WHEEL_SIZE);
          t.wheels_constant.liftable = self.get_value_vec(WHEEL_SIZE);
          t.wheels.on_ground = self.get_value_vec(WHEEL_SIZE);

          t.attached = self.get_value();
          self.pad_offset(3);

          t.wheels.substance = self.get_value_vec(WHEEL_SIZE);
          t.wheels_constant.count = self.get_value();

          t.damage.cargo = self.get_value();
          t.damage.chassis = self.get_value();
          t.damage.wheels_avg = self.get_value();
          t.damage.body = self.get_value();

          t.wheels.suspension_deflection = self.get_value_vec(WHEEL_SIZE);
          t.wheels.velocity = self.get_value_vec(WHEEL_SIZE);
          t.wheels.steering = self.get_value_vec(WHEEL_SIZE);
          t.wheels.rotation = self.get_value_vec(WHEEL_SIZE);
          t.wheels.lift = self.get_value_vec(WHEEL_SIZE);
          t.wheels.lift_offset = self.get_value_vec(WHEEL_SIZE);
          t.wheels_constant.radius = self.get_value_vec(WHEEL_SIZE);

          t.acceleration.linear_velocity = self.get_vector();
          t.acceleration.angular_velocity = self.get_vector();
          t.acceleration.linear_acceleration = self.get_vector();
          t.acceleration.angular_acceleration = self.get_vector();

          t.hook = self.get_vector();

          t.wheels_constant.position = {
            let mut temp_pos: Vec<Vector<f32>> = vec![Vector::default(); WHEEL_SIZE];

            temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.x = self.get_value());
            temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.y = self.get_value());
            temp_pos.iter_mut().for_each(|p: &mut Vector<f32>| p.z = self.get_value());

            temp_pos
          };

          self.pad_offset(4);

          t.position = self.get_placement();

          t.id = self.get_string(None);
          t.cargo_accessory_id = self.get_string(None);
          t.body_type = self.get_string(None);
          t.brand_id = self.get_string(None);
          t.brand = self.get_string(None);
          t.name = self.get_string(None);
          t.chain_type = self.get_string(None);
          t.license_plate = self.get_string(None);
          t.license_plate_country = self.get_string(None);
          t.license_plate_country_id = self.get_string(None);
        }
      );

      temp_trailers.retain(
        |x: &Trailer| {
          !x.cargo_accessory_id.is_empty() ||
          !x.id.is_empty() ||
          !x.body_type.is_empty() ||
          !x.license_plate.is_empty()
        }
      );

      temp_trailers
    };


    telemetry_data
  }

  pub(self) fn next_offset_area(self: &mut Self) {
    self.offset_area += 1;
    self.offset = OFFSET_AREAS[self.offset_area];
  }

  pub(self) fn pad_offset(self: &mut Self, value: isize) {
    self.offset += value;
  }

  pub(self) fn get_value<T>(self: &mut Self) -> T {
    unsafe {
      let res: T = (self.data_pointer as *const T).byte_offset(self.offset).read_volatile();
      self.pad_offset(size_of::<T>() as isize);
      res
    }
  }

  pub(self) fn get_value_vec<T>(self: &mut Self, length: usize) -> Vec<T> {
    let mut res: Vec<T> = Vec::with_capacity(length);
    (0..length).for_each(
      |_| {
        res.push(self.get_value());
      }
    );
    res
  }

  pub(self) fn get_vector<T>(self: &mut Self) -> Vector<T> {
    Vector::<T> {
      x: self.get_value(),
      y: self.get_value(),
      z: self.get_value()
    }
  }

  pub(self) fn get_eular<T>(self: &mut Self) -> Eular<T> {
    Eular {
      heading: self.get_value(),
      pitch: self.get_value(),
      roll: self.get_value()
    }
  }

  pub(self) fn get_placement<T>(self: &mut Self) -> Placement<T> {
    Placement::<T> {
      position: self.get_vector(),
      orientation: self.get_eular()
    }
  }

  pub(self) fn get_string(self: &mut Self, length: Option<usize>) -> String {
    unsafe {
      let res: String = str::from_utf8(
        slice::from_raw_parts(
          (self.data_pointer as *const u8).byte_offset(self.offset),
          length.unwrap_or(STRING_SIZE)
        )
      ).unwrap_or_default().trim().replace("\0", "");
      self.pad_offset(length.unwrap_or(STRING_SIZE) as isize);
      res
    }
  }
}