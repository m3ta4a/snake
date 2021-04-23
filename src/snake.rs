use dynamo_lib::geometry::quad::Quad;

pub struct Snake {
  pub body: Vec<Quad>,
  pub score: u32,
  pub visible: bool,
}

impl Snake {
  pub fn new(position: cgmath::Vector2<f32>) -> Snake {
    Snake {
      body: vec![Quad::new(position, (0.0, 0.0).into())],
      score: 0,
      visible: false,
    }
  }

  // pub fn contains(&self, pellet: &Pellet) -> bool {
  //   let radii = self.size() * 0.5;
  //   let min = self.position() - radii;
  //   let max = self.position() + radii;

  //   let b_radii = cgmath::Vector2 {
  //     x: ball.radius(),
  //     y: ball.radius(),
  //   };
  //   let b_min = ball.position() - b_radii;
  //   let b_max = ball.position() + b_radii;

  //   min.x < b_max.x && max.x > b_min.x && min.y < b_max.y && max.y > b_min.y
  // }
}
