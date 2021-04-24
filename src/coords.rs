use crate::snake::Snake;
use cgmath::Vector2;

pub fn snake_coordinates(snake: &Snake, position: Vector2<f32>) -> (i32, i32) {
  let x_100 = (position.x * 100.0).round();
  let sx_100 = (snake.segment_size.x * 100.0).round();

  let y_100 = (position.y * 100.0).round();
  let sy_100 = (snake.segment_size.y * 100.0).round();

  let x = (x_100 / sx_100) as i32;
  let y = (y_100 / sy_100) as i32;

  (x, y)
}

pub fn screen_coordinates(snake: &Snake, snake_coords: (i32, i32)) -> Vector2<f32> {
  Vector2 {
    x: snake_coords.0 as f32 * snake.segment_size.x,
    y: snake_coords.1 as f32 * snake.segment_size.y,
  }
}
