use super::shifter_type::ShifterType;


#[derive(Debug, Clone, Default)]
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
