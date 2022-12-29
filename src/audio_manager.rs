use ggez::Context;
use ggez::audio::*;

pub struct AudioManager {
    goal_sound: Source,
    hit_sound: Source,
}

impl AudioManager {
    pub fn new(context: &Context) -> AudioManager {
        AudioManager {
            goal_sound: Source::new(context, "/goal.ogg").unwrap(),
            hit_sound: Source::new(context, "/hit.ogg").unwrap(),
        }
    }

    pub fn play_hit(&mut self, ctx: &Context) {
        self.hit_sound.play_detached(ctx).unwrap()
    }

    pub fn play_goal(&mut self, ctx: &Context) {
        self.goal_sound.play_detached(ctx).unwrap()
    }
}