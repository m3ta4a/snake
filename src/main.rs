use dynamo_lib::start;

mod snake_game;

use snake_game::SnakeGame;

fn main() {
  let snake_game = SnakeGame::new();
  start("Snake", Box::new(snake_game));
}
