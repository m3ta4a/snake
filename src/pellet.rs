use dynamo_lib::geometry::quad::Quad;

pub struct Pellet {
  pub quad: Quad,
  pub visible: bool,
}

impl Pellet {
  pub fn new(position: cgmath::Vector2<f32>, radius: f32) -> Pellet {
    Pellet {
      quad: Quad::new(position, (radius, radius).into()),
      visible: false,
    }
  }

  pub fn update_position(&mut self, position: cgmath::Vector2<f32>) {
    self.quad = Quad::new(position, self.quad.size);
  }
}
