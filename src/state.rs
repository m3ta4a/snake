use crate::pellet::Pellet;
use crate::snake::Snake;
use dynamo_lib::geometry::quad::Quad;
use dynamo_lib::geometry::Geometry;
use dynamo_lib::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
  MainMenu,
  Playing,
  Paused,
  GameOver,
  Quitting,
}

pub struct SnakeText {
  pub render_text: RenderText,
  pub visible: bool,
}

impl SnakeText {
  pub fn focused(&self) -> bool {
    self.render_text.focused
  }

  pub fn set_focus(&mut self, focused: bool) {
    self.render_text.focused = focused;
  }
}

pub struct State {
  pub game_state: GameState,
  pub walls: Vec<Quad>,
  pub snake: Snake,
  pub pellet: Pellet,
  pub title_text: SnakeText,
  pub play_button: SnakeText,
  pub quit_button: SnakeText,
  pub score: SnakeText,
  pub win_text: SnakeText,
  // window_size: (f32, f32),
}

impl State {
  pub fn new() -> Self {
    Self {
      game_state: GameState::MainMenu,
      walls: vec![
        // Unclear why the size needs to be 4 to stretch the coordinate system
        // x & y min are -1, x & y max are +1
        // where does 4 come from?? All I know is 2 only goes halfway across the window
        Quad::new((-1.0, -1.0).into(), (4.0, 0.02).into()),
        Quad::new((-1.0, 1.0).into(), (4.0, 0.02).into()),
        Quad::new((1.0, -1.0).into(), (0.02, 4.0).into()),
        Quad::new((-1.0, -1.0).into(), (0.02, 4.0).into()),
      ],
      snake: Snake::new((0.0, 0.0).into(), (0.04, 0.04).into()),
      pellet: Pellet::new((0.0, 0.0).into(), 0.04),
      title_text: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (20.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("SNAKE"),
          size: 64.0,
          ..Default::default()
        },
      },
      play_button: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 100.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Play"),
          size: 32.0,
          ..Default::default()
        },
      },
      quit_button: SnakeText {
        visible: false,
        render_text: RenderText {
          position: (40.0, 160.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("Quit"),
          size: 32.0,
          ..Default::default()
        },
      },
      score: SnakeText {
        visible: false,
        render_text: RenderText {
          // position: (render.width() * 0.75, 20.0).into(),
          position: (120.0, 20.0).into(),
          color: (1.0, 1.0, 1.0, 1.0).into(),
          text: String::from("0"),
          size: 32.0,
          ..Default::default()
        },
      },
      win_text: SnakeText {
        visible: false,
        render_text: RenderText {
          // position: (render.width() * 0.5, render.height() * 0.5).into(),
          position: (200.0, 200.0).into(),
          bounds: (UNBOUNDED_F32, UNBOUNDED_F32).into(),
          size: 32.0,
          centered: true,
          ..Default::default()
        },
      },
    }
  }

  pub fn initialize(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  pub fn update(&self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
    self.update_geometry(geometry);
    self.update_text(text_renderer);
  }

  fn update_geometry(&self, geometry: &mut Geometry) {
    if self.snake.visible {
      for quad in self.walls.iter() {
        geometry.push_quad(&quad);
      }

      for quad in self.snake.body.iter() {
        geometry.push_quad(&quad);
      }
    }

    if self.pellet.visible {
      geometry.push_quad(&self.pellet.quad);
    }
  }

  fn update_text(&self, text_renderer: &mut TextRenderer) {
    for text in vec![
      &self.title_text,
      &self.play_button,
      &self.quit_button,
      &self.score,
      &self.win_text,
    ]
    .iter()
    {
      if text.visible {
        text_renderer.push_render_text(text.render_text.clone());
      }
    }
  }

  pub fn pause_game(&mut self) {
    if self.game_state == GameState::Playing {
      self.game_state = GameState::Paused;
    }
  }
}
