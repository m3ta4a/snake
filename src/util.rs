#![macro_use]

pub const SNAKE_SPEED: f32 = 0.0025;

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}
