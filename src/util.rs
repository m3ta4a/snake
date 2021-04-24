#![macro_use]

pub const SNAKE_SPEED: f32 = 0.01;

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
