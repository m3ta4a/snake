#![macro_use]

pub const STARTING_SNAKE_SPEED: f32 = 0.005;
pub const SNAKE_SPEED_INC: f32 = 0.00075;

pub enum Direction {
  None,
  Up,
  Down,
  Left,
  Right,
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}
