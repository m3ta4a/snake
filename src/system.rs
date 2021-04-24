use crate::any;
use crate::input::Input;
use crate::snake_game::*;
use crate::state::*;
use crate::util;

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
    state.snake.score = 0;
    state.snake.update_position((0.0, 0.0).into());
    state.snake.update_direction((0.0, 0.0).into());
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
  }
}

#[derive(Debug)]
pub struct SnakeSystem;

impl System for SnakeSystem {
  fn update_state(&self, input: &mut Input, state: &mut State, _events: &mut Vec<Event>) {
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
      if state.snake.collides(quad) {
        state.game_state = GameState::GameOver;
      }
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

    state.score.render_text.text = format!("{}", state.snake.score);

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
