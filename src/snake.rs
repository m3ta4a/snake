use crate::coords;
use cgmath::Vector2;
use dynamo_lib::geometry::quad::Quad;

pub struct Snake {
  pub body: Vec<Quad>,
  pub segment_size: Vector2<f32>,
  pub position: Vector2<f32>,
  pub direction: Vector2<f32>,
  pub score: u32,
  pub visible: bool,
}

impl Snake {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Snake {
    Snake {
      body: vec![Quad::new(position, size)],
      segment_size: size,
      position: position,
      direction: (0.0, 0.0).into(),
      score: 0,
      visible: false,
    }
  }

  pub fn grow_body(&mut self) {
    let last_segment = self.body[self.body.len() - 1].clone();

    self.body.append(&mut vec![last_segment]);
  }

  pub fn update_direction(&mut self, direction: Vector2<f32>) {
    self.direction = direction;
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

  pub fn consumes(&self, quad: &Quad) -> bool {
    let snake_coords = coords::snake_coordinates(self.segment_size, self.position);
    let quad_coords = coords::snake_coordinates(quad.size, quad.position);

    snake_coords.0 == quad_coords.0 && snake_coords.1 == quad_coords.1
  }
}
