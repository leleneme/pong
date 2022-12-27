use ggez::audio::*;
use ggez::glam::Vec2;
use ggez::graphics::Mesh;
use ggez::Context;
use rand::{self, thread_rng, Rng};

use crate::constants::*;
use crate::paddle::*;

pub struct Ball {
    pub mesh: Mesh,
    pub position: Vec2,
    pub velocity: Vec2,
    hit_sound: Source,
}

impl Ball {
    pub fn new(ctx: &Context, mesh: Mesh, position: Vec2) -> Ball {
        Ball {
            mesh,
            position,
            velocity: Ball::random_velocity(),
            hit_sound: Source::new(ctx, "/hit.ogg").unwrap(),
        }
    }

    pub fn random_velocity() -> Vec2 {
        let mut rng = thread_rng();
        let x = match rng.gen_bool(0.5) {
            true => BALL_SPEED,
            false => -BALL_SPEED,
        };

        let y = match rng.gen_bool(0.5) {
            true => BALL_SPEED,
            false => -BALL_SPEED,
        };

        Vec2::new(x, y)
    }

    pub fn reset(&mut self, ctx: &Context) {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        self.position.x = (screen_w - PADDLE_W) * 0.5;
        self.position.y = (screen_h - PADDLE_W) * 0.5;
        self.velocity = Ball::random_velocity();
    }

    pub fn update_ball(&mut self, ctx: &Context, paddle_1: &Paddle, paddle_2: &Paddle) {
        let delta = ctx.time.delta().as_secs_f32();

        self.position += self.velocity * delta;

        let (_, screen_h) = ctx.gfx.drawable_size();

        let mut did_collide = false;

        // bouce on celling and roof
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y = self.velocity.y.abs();

            did_collide = true;
        } else if self.position.y + BALL_W >= screen_h {
            self.position.y = screen_h - BALL_W;
            self.velocity.y = -self.velocity.y.abs();

            did_collide = true;
        }

        if self.is_colliding(paddle_1) {
            self.velocity.x = self.velocity.x.abs();

            did_collide = true;
        } else if self.is_colliding(paddle_2) {
            self.velocity.x = -self.velocity.x.abs();

            did_collide = true;
        }

        if did_collide {
            self.hit_sound.play_detached(ctx).unwrap()
        }
    }

    pub fn is_colliding(&self, paddle: &Paddle) -> bool {
        let (ball_x, ball_y) = (self.position.x, self.position.y);
        let (ball_w, ball_h) = (BALL_W, BALL_W);

        let (paddle_x, paddle_y) = (paddle.position.x, paddle.position.y);
        let (paddle_w, paddle_h) = (PADDLE_W, PADDLE_H);

        ball_x < paddle_x + paddle_w
            && ball_x + ball_w > paddle_x
            && ball_y < paddle_y + paddle_h
            && ball_y + ball_h > paddle_y
    }
}
