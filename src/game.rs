use ggez::audio::*;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{Canvas, Color, DrawMode, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::ball::Ball;
use crate::constants::*;
use crate::paddle::{Direction, Paddle};

pub struct MainState {
    player_1: Paddle,
    player_2: Paddle,
    ball: Ball,
    score: (u32, u32),
    middle_bar: Mesh,
    middle_position: Vec2,
    goal_sound: Source,
}

fn build_rect_mesh(ctx: &Context, width: f32, height: f32, color: Color) -> Mesh {
    let rect = Rect::new(0.0, 0.0, width, height);
    Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color).expect("Could not build mesh!")
}

impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let paddle_mesh = build_rect_mesh(ctx, PADDLE_W, PADDLE_H, Color::WHITE);
        let ball_mesh = build_rect_mesh(ctx, PADDLE_W, PADDLE_W, Color::WHITE);

        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let paddle_middle_y = (screen_h * 0.5) - (PADDLE_H * 0.5);
        let ball_middle_vec = Vec2::new((screen_w - PADDLE_W) * 0.5, (screen_h - PADDLE_W) * 0.5);

        let middle_line_mesh = build_rect_mesh(ctx, 2.0, screen_h, Color::WHITE);

        let state = MainState {
            player_1: Paddle {
                mesh: paddle_mesh.clone(),
                position: Vec2::new(0.0 + SCREEN_PADDING, paddle_middle_y),
            },
            player_2: Paddle {
                mesh: paddle_mesh,
                position: Vec2::new(screen_w - SCREEN_PADDING * 2.0, paddle_middle_y),
            },
            ball: Ball::new(ctx, ball_mesh, ball_middle_vec),
            middle_bar: middle_line_mesh,
            middle_position: Vec2::new(screen_w * 0.5, 0.0),
            score: (0, 0),
            goal_sound: Source::new(ctx, "/goal.ogg").unwrap(),
        };

        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_pressed(KeyCode::W) {
            self.player_1.move_paddle(ctx, Direction::Up);
        } else if k_ctx.is_key_pressed(KeyCode::S) {
            self.player_1.move_paddle(ctx, Direction::Down);
        }

        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.player_2.move_paddle(ctx, Direction::Up);
        } else if k_ctx.is_key_pressed(KeyCode::Down) {
            self.player_2.move_paddle(ctx, Direction::Down);
        }

        self.ball.update_ball(ctx, &self.player_1, &self.player_2);

        if k_ctx.is_key_pressed(KeyCode::R) {
            self.ball.reset(ctx);
            self.score = (0, 0);
        }

        let (screen_w, _) = ctx.gfx.drawable_size();

        let mut did_goal = false;
        if self.ball.position.x <= 0.0 {
            self.score.1 += 1;
            self.ball.reset(ctx);

            did_goal = true;
        } else if self.ball.position.x >= (screen_w - BALL_W) {
            self.score.0 += 1;
            self.ball.reset(ctx);

            did_goal = true;
        }

        if did_goal {
            self.goal_sound.play_detached(ctx).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.player_1.mesh, self.player_1.position);
        canvas.draw(&self.player_2.mesh, self.player_2.position);
        canvas.draw(&self.ball.mesh, self.ball.position);
        canvas.draw(&self.middle_bar, self.middle_position);

        let screen_w = ctx.gfx.drawable_size().0;
        let mut score_text: Text = Text::new(format!("{}    {}", self.score.0, self.score.1));
        score_text.set_scale(44.0);
        let text_width = score_text.measure(ctx)?.x;

        let coords = [screen_w * 0.5 - text_width * 0.5, 10.0];

        canvas.draw(&score_text, coords);

        canvas.finish(ctx)?;
        Ok(())
    }
}
