use dynamo_lib::geometry::Geometry;
use dynamo_lib::keyboard::*;
use dynamo_lib::renderer::render_text::TextRenderer;
use dynamo_lib::sound::SoundSystem;
use dynamo_lib::Game;

pub struct SnakeGame {}

impl SnakeGame {
  pub fn new() -> Self {
    Self {}
  }
}

impl Game for SnakeGame {
  fn initialize(
    &mut self,
    _geometry: &mut Geometry,
    _text_renderer: &mut TextRenderer,
    _sound_system: &SoundSystem,
    _window_size: (f32, f32),
  ) {
  }

  fn update(
    &mut self,
    _geometry: &mut Geometry,
    _text_renderer: &mut TextRenderer,
    _sound_system: &SoundSystem,
  ) {
  }

  fn process_keyboard(&mut self, _input: KeyboardInput) {}

  fn is_quitting(&self) -> bool {
    false
  }
}
