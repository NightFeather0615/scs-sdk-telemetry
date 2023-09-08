#[derive(Debug, Clone, Default)]
pub struct Control {
  pub raw_input: RawInput,
  pub game_input: GameInput
}

#[derive(Debug, Clone, Default)]
pub struct RawInput {
  pub steering: f32,
  pub throttle: f32,
  pub brake: f32,
  pub clutch: f32
}

#[derive(Debug, Clone, Default)]
pub struct GameInput {
  pub steering: f32,
  pub throttle: f32,
  pub brake: f32,
  pub clutch: f32
}
