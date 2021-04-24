use cgmath::Vector2;
use dynamo_lib::geometry::quad::Quad;

pub struct Snake {
  pub body: Vec<Quad>,
  pub segment_size: Vector2<f32>,
  pub head_pos: Vector2<f32>,
  pub velocity: Vector2<f32>,
  pub score: u32,
  pub visible: bool,
}

impl Snake {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Snake {
    Snake {
      body: vec![Quad::new(position, size)],
      segment_size: size,
      head_pos: position,
      velocity: (0.0, 0.0).into(),
      score: 0,
      visible: false,
    }
  }

  pub fn update_velocity(&mut self, velocity: Vector2<f32>) {
    self.velocity = velocity;
  }

  pub fn position(&self) -> Vector2<f32> {
    self.head_pos
  }

  pub fn update_position(&mut self, position: Vector2<f32>) {
    let head = self.head();

    self.head_pos = position;

    let new_x = if self.is_increment(position.x, self.segment_size.x) {
      self.round(position.x)
    } else {
      head.position.x
    };
    let new_y = if self.is_increment(position.y, self.segment_size.y) {
      self.round(position.y)
    } else {
      head.position.y
    };

    // println!("{:?}", (new_x, new_y));

    self.body = vec![Quad::new((new_x, new_y).into(), head.size)];
  }

  fn head(&self) -> Quad {
    self.body[0]
  }

  fn round(&self, val: f32) -> f32 {
    (val * 100.0).round() / 100.0
  }

  fn is_increment(&self, val: f32, inc: f32) -> bool {
    let val_100 = val * 100.0;
    let inc_100 = inc * 100.0;
    (val_100 % inc_100).round() == 0.0
  }

  // pub fn contains(&self, pellet: &Pellet) -> bool {
  //   let radii = self.size() * 0.5;
  //   let min = self.position() - radii;
  //   let max = self.position() + radii;

  //   let b_radii = Vector2 {
  //     x: ball.radius(),
  //     y: ball.radius(),
  //   };
  //   let b_min = ball.position() - b_radii;
  //   let b_max = ball.position() + b_radii;

  //   min.x < b_max.x && max.x > b_min.x && min.y < b_max.y && max.y > b_min.y
  // }
}
