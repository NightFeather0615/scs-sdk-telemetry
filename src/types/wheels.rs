use super::vector::Vector;


#[derive(Debug, Clone, Default)]
pub struct WheelsConstants {
  pub count: u32,
  pub radius: Vec<f32>,
  pub simulated: Vec<bool>,
  pub powered: Vec<bool>,
  pub liftable: Vec<bool>,
  pub steerable: Vec<bool>,
  pub position: Vec<Vector<f32>>,
}
