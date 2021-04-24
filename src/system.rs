use crate::any;
use crate::input::Input;
use crate::snake_game::*;
use crate::state::*;
use crate::util;
use rand::Rng;

pub trait System {
  #[allow(unused_variables)]
  fn start(&mut self, game: &mut State) {}
  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>);
}

pub struct VisibilitySystem;
impl System for VisibilitySystem {
  fn update_state(&self, _input: &mut Input, state: &mut State, _events: &mut Vec<Event>) {
    let is_in_game = any!(state.game_state, GameState::Playing, GameState::GameOver);
    state.snake.visible = is_in_game;
    state.score.visible = is_in_game;
    state.pellet.visible = is_in_game;

    state.title_text.visible = state.game_state == GameState::MainMenu;
    state.play_button.visible = state.game_state == GameState::MainMenu;
    state.quit_button.visible = state.game_state == GameState::MainMenu;

    state.win_text.visible = state.game_state == GameState::GameOver;
  }
}

#[derive(Debug)]
pub struct MenuSystem;

impl System for MenuSystem {
  fn start(&mut self, state: &mut State) {
    state.play_button.render_text.focused = true;
    state.quit_button.render_text.focused = false;
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;

      input.esc_pressed = false;
    }

    if state.play_button.focused() && input.ui_down_pressed() {
      events.push(Event::FocusChanged);
      state.play_button.set_focus(false);
      state.quit_button.set_focus(true);
    } else if state.quit_button.focused() && input.ui_up_pressed() {
      events.push(Event::FocusChanged);
      state.quit_button.set_focus(false);
      state.play_button.set_focus(true);
    }

    if state.play_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Playing;
    } else if state.quit_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;
    }
  }
}

#[derive(Debug)]
pub struct PlaySystem;

impl System for PlaySystem {
  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      input.clear();
      events.push(Event::ButtonPressed);
      state.game_state = GameState::MainMenu;

      input.esc_pressed = false;
    }

    state.score.render_text.text = format!("Score: {}", state.snake.score);
  }
}

#[derive(Debug)]
pub struct SnakeSystem;

impl System for SnakeSystem {
  fn start(&mut self, state: &mut State) {
    state.snake.score = 0;
    state.snake.update_position((0.0, 0.0).into());
    state.snake.update_direction((0.0, 0.0).into());

    let random_position = self.random_position(state);
    state.pellet.update_position(random_position.into())
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    state
      .snake
      .update_position(state.snake.position() + state.snake.direction * util::SNAKE_SPEED);

    if input.up_pressed {
      state.snake.update_direction((0.0, 1.0).into());
    }
    if input.down_pressed {
      state.snake.update_direction((0.0, -1.0).into());
    }
    if input.right_pressed {
      state.snake.update_direction((1.0, 0.0).into());
    }
    if input.left_pressed {
      state.snake.update_direction((-1.0, 0.0).into());
    }

    for quad in state.walls.iter() {
      if state.snake.intersects(quad) {
        events.push(Event::SnakeCrashed);
        state.game_state = GameState::GameOver;
      }
    }

    if state.snake.consumes(&state.pellet.quad) {
      state.snake.score = state.snake.score + 1;
      events.push(Event::Score(state.snake.score));

      let random_position = self.random_position(state);
      state.pellet.update_position(random_position.into())
    }
  }
}

impl SnakeSystem {
  fn random_position(&self, state: &mut State) -> cgmath::Vector2<f32> {
    let mut rng = rand::thread_rng();

    // Unclear why the dimensions of a -1 to 1 coordinate system are 4, but here we are
    // Divide the size of the window by the size of a segment of the snake to get the number of cols and rows
    let upper_bound_x = (4.0 / state.snake.segment_size.x).round() as i64;
    let upper_bound_y = (4.0 / state.snake.segment_size.y).round() as i64;

    let random_num_x = rng.gen_range(-upper_bound_x..upper_bound_x);
    let random_num_y = rng.gen_range(-upper_bound_y..upper_bound_y);

    // Divide by the limit to get a fraction, multiply by 4 to ensure the mantissa
    // is a multiple of four to match snake "grid" positions
    let x_4 = 4.0 * (random_num_x as f32 / upper_bound_x as f32);
    let y_4 = 4.0 * (random_num_y as f32 / upper_bound_y as f32);

    let i_x = x_4 as i32;
    let i_y = y_4 as i32;

    // Get just the mantissa
    let f_x = x_4 - i_x as f32;
    let f_y = y_4 - i_y as f32;

    (f_x, f_y).into()
  }
}

pub struct GameOverSystem {
  last_time: std::time::Instant,
}

impl GameOverSystem {
  pub fn new() -> Self {
    Self {
      last_time: std::time::Instant::now(),
    }
  }
}

impl System for GameOverSystem {
  fn start(&mut self, state: &mut State) {
    self.last_time = std::time::Instant::now();

    state.win_text.render_text.text = String::from("Game Over")
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Quitting;

      input.esc_pressed = false;
    }

    let current_time = std::time::Instant::now();
    let delta_time = current_time - self.last_time;
    if delta_time.as_secs_f32() > 5.0 {
      state.game_state = GameState::MainMenu;
    }
  }
}
