use crate::coords;
use crate::util;
use crate::util::Direction;
use crate::util::Direction::*;
use cgmath::Vector2;
use dynamo_lib::geometry::quad::Quad;

pub struct Snake {
  pub body: Vec<Quad>,
  pub segment_size: Vector2<f32>,
  pub position: Vector2<f32>,
  pub direction: Direction,
  pub speed: f32,
  pub score: u32,
  pub visible: bool,
}

impl Snake {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Snake {
    Snake {
      body: vec![Quad::new(position, size)],
      segment_size: size,
      position: position,
      direction: None,
      speed: util::STARTING_SNAKE_SPEED,
      score: 0,
      visible: false,
    }
  }

  pub fn reset(&mut self) {
    self.score = 0;
    self.speed = util::STARTING_SNAKE_SPEED;
    self.update_position((0.0, 0.0).into());
    self.update_direction(None);
    self.reset_body();
  }

  pub fn reset_body(&mut self) {
    self.body = vec![self.head()];
  }

  pub fn grow_body(&mut self) {
    let mut last_segment = self.body[self.body.len() - 1].clone();
    last_segment.position = (
      last_segment.position.x + -1.0 * self.direction().x * self.segment_size.x,
      last_segment.position.y + -1.0 * self.direction().y * self.segment_size.y,
    )
      .into();

    self.body.append(&mut vec![last_segment]);
  }

  pub fn update_direction(&mut self, direction: Direction) {
    self.direction = direction;
  }

  pub fn direction(&self) -> Vector2<f32> {
    match self.direction {
      None => (0.0, 0.0).into(),
      Up => (0.0, 1.0).into(),
      Down => (0.0, -1.0).into(),
      Left => (-1.0, 0.0).into(),
      Right => (1.0, 0.0).into(),
    }
  }

  pub fn position(&self) -> Vector2<f32> {
    self.position
  }

  pub fn update_position(&mut self, position: Vector2<f32>) {
    self.position = position;

    let head = self.head();
    let cur_snake_coords = coords::snake_coordinates(self.segment_size, head.position);
    let new_snake_coords = coords::snake_coordinates(self.segment_size, position);

    if cur_snake_coords.0 != new_snake_coords.0 || cur_snake_coords.1 != new_snake_coords.1 {
      let new_screen_coords = coords::screen_coordinates(self.segment_size, new_snake_coords);

      let new_head = vec![Quad::new(new_screen_coords.into(), head.size)];
      let old_body = &self.body[0..self.body.len() - 1];

      self.body = [&new_head[..], old_body].concat();
    }
  }

  fn head(&self) -> Quad {
    self.body[0]
  }

  pub fn intersects(&self, quad: &Quad) -> bool {
    let snake_coords = coords::snake_coordinates(self.segment_size, self.position);
    let screen_coords = coords::screen_coordinates(self.segment_size, snake_coords);

    let radii = self.segment_size * 0.5;
    let min = screen_coords - radii;
    let max = screen_coords + radii;

    let b_radii = Vector2 {
      x: quad.size.x,
      y: quad.size.y,
    };
    let b_min = quad.position - b_radii;
    let b_max = quad.position + b_radii;

    min.x < b_max.x && max.x > b_min.x && min.y < b_max.y && max.y > b_min.y
  }

  pub fn collides(&self, quad: &Quad) -> bool {
    let snake_coords = coords::snake_coordinates(self.segment_size, self.position);
    let quad_coords = coords::snake_coordinates(quad.size, quad.position);

    snake_coords.0 == quad_coords.0 && snake_coords.1 == quad_coords.1
  }
}
