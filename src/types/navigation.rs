use super::movement::Movement;


#[derive(Debug, Clone, Default)]
pub struct Navigation {
  pub navigation_distance: f32,
  pub navigation_time: f32,
  pub speed_limit: Movement
}
