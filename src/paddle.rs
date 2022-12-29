use ggez::glam::Vec2;
use ggez::graphics::Mesh;
use ggez::Context;

use crate::constants::*;

pub struct Paddle {
    pub mesh: Mesh,
    pub position: Vec2,
}

pub enum Direction {
    Up,
    Down,
}

impl Paddle {
    pub fn new(mesh: Mesh, x: f32, y: f32) -> Paddle {
        Paddle {
            mesh,
            position: Vec2::new(x, y),
        }
    }

    pub fn move_paddle(&mut self, ctx: &Context, direction: Direction) {
        let move_vec: Vec2 = match direction {
            Direction::Up => Vec2::new(0.0, -PADDLE_SPEED),
            Direction::Down => Vec2::new(0.0, PADDLE_SPEED),
        };

        let delta = ctx.time.delta().as_secs_f32();
        self.position += move_vec * delta;

        let (_, screen_h) = ctx.gfx.drawable_size();
        let low = 0.0 + SCREEN_PADDING;
        let high = screen_h - (SCREEN_PADDING + PADDLE_H);

        self.position.y = clamp(self.position.y, low, high);
    }
}

fn clamp<T>(value: T, low: T, high: T) -> T
where
    T: PartialOrd<T>,
{
    if value < low {
        return low;
    } else if value > high {
        return high;
    }

    value
}
