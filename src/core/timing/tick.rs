use bevy::prelude::*;
use crate::core::timing::GameTime;

/// Controls simulation speed
#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub enum GameSpeed {
    Paused,
    Normal,
    Fast,
    VeryFast,
}

impl GameSpeed {
    pub fn multiplier(&self) -> f32 {
        match self {
            GameSpeed::Paused => 0.0,
            GameSpeed::Normal => 1.0,
            GameSpeed::Fast => 2.0,
            GameSpeed::VeryFast => 5.0,
        }
    }
}

/// Timer controlling day progression
#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
}

/// Advances game time (ONLY authority over time)
pub fn tick_driver(
    time: Res<Time>,
    speed: Res<GameSpeed>,
    mut tick_timer: ResMut<TickTimer>,
    mut game_time: ResMut<GameTime>,
) {
    let multiplier = speed.multiplier();

    if multiplier == 0.0 {
        return;
    }

    let scaled_delta = time.delta().mul_f32(multiplier);

    if tick_timer.timer.tick(scaled_delta).just_finished() {
        game_time.advance_day();
    }
}