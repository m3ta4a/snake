use crate::any;
use crate::coords;
use crate::input::Input;
use crate::snake_game::*;
use crate::state::*;
use crate::util;
use crate::util::Direction::*;
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

    state.title_text.visible =
      state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
    state.play_button.visible =
      state.game_state == GameState::MainMenu || state.game_state == GameState::Paused;
    state.quit_button.visible = state.game_state == GameState::MainMenu;

    state.win_text.visible = state.game_state == GameState::GameOver;
  }
}

#[derive(Debug)]
pub struct MenuSystem;

impl System for MenuSystem {
  fn start(&mut self, state: &mut State) {
    state.title_text.render_text.text = String::from("SNAKE");
    state.play_button.render_text.text = String::from("Play");

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
  fn start(&mut self, state: &mut State) {
    state.snake.reset();

    let random_position = self.random_position(state);
    state.pellet.update_position(random_position.into())
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if input.esc_pressed {
      input.clear();
      events.push(Event::ButtonPressed);
      state.game_state = GameState::MainMenu;

      input.esc_pressed = false;
    }

    state.score.render_text.text = format!("Score: {}", state.snake.score);

    state
      .snake
      .update_position(state.snake.position() + state.snake.direction() * state.snake.speed);

    if input.up_pressed && !matches!(state.snake.direction, Down) {
      state.snake.update_direction(Up);
    }
    if input.down_pressed && !matches!(state.snake.direction, Up) {
      state.snake.update_direction(Down);
    }
    if input.right_pressed && !matches!(state.snake.direction, Left) {
      state.snake.update_direction(Right);
    }
    if input.left_pressed && !matches!(state.snake.direction, Right) {
      state.snake.update_direction(Left);
    }

    for quad in state.walls.iter() {
      if state.snake.intersects(quad) {
        events.push(Event::SnakeCrashed);
        state.game_state = GameState::GameOver;
      }
    }

    let body_after_head = &state.snake.body[1..];

    for quad in body_after_head.iter() {
      if state.snake.collides(quad) {
        events.push(Event::SnakeCrashed);
        state.game_state = GameState::GameOver;
      }
    }

    if state.snake.collides(&state.pellet.quad) {
      state.snake.score = state.snake.score + 1;
      events.push(Event::Score(state.snake.score));

      state.snake.grow_body();
      state.snake.speed += util::SNAKE_SPEED_INC;

      let random_position = self.random_position(state);
      state.pellet.update_position(random_position.into());
    }
  }
}

impl PlaySystem {
  fn random_position(&self, state: &mut State) -> cgmath::Vector2<f32> {
    let mut rng = rand::thread_rng();

    let limit_x = (1.0 / state.snake.segment_size.x).round() as i32 - 1;
    let limit_y = (1.0 / state.snake.segment_size.y).round() as i32 - 1;

    let rand_x = rng.gen_range(-limit_x..limit_x);
    let rand_y = rng.gen_range(-limit_y..limit_y);

    coords::screen_coordinates(state.snake.segment_size, (rand_x, rand_y))
  }
}

#[derive(Debug)]
pub struct PauseSystem;

impl System for PauseSystem {
  fn start(&mut self, state: &mut State) {
    state.title_text.render_text.text = String::from("Paused");
    state.play_button.render_text.text = String::from("Resume");
    state.play_button.render_text.focused = true;
  }

  fn update_state(&self, input: &mut Input, state: &mut State, events: &mut Vec<Event>) {
    if state.play_button.focused() && input.enter_pressed {
      events.push(Event::ButtonPressed);
      state.game_state = GameState::Playing;
    }
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
