use core::{f32};

use bevy::prelude::*;
use crate::{core::timing::GameTime, data::{DAYS_IN_MONTH, MONTHS_IN_YEAR, STARTING_YEAR}};


#[derive(Message)]
pub struct TickMessage;

#[derive(Message)]
pub struct DailyTick;

#[derive(Message)]
pub struct MonthlyTick;

#[derive(Message)]
pub struct YearlyTick;


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

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
}

pub fn tick_driver(
    time: Res<Time>,
    speed: Res<GameSpeed>,
    mut tick_timer: ResMut<TickTimer>,
    mut game_time: ResMut<GameTime>,
    mut tick_messages: MessageWriter<TickMessage>,
) {
    let multiplier = speed.multiplier();

    if multiplier == 0.0 {
        return;
    }

    // scale time progression
    let scaled_delta = time.delta().mul_f32(multiplier);

    if tick_timer.timer.tick(scaled_delta).just_finished() {
        game_time.advance_day();
        tick_messages.write(TickMessage);
    }
}
impl Default for LastGameTime {
    fn default() -> Self {
        Self {
            day: DAYS_IN_MONTH,
            month: MONTHS_IN_YEAR,
            year: STARTING_YEAR - 1 
        }
    }
}


#[derive(Resource)]
pub struct LastGameTime {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}


pub fn time_dispatcher(
    game_time: Res<GameTime>,
    mut last: ResMut<LastGameTime>,
    mut daily: MessageWriter<DailyTick>,
    mut monthly: MessageWriter<MonthlyTick>,
    mut yearly: MessageWriter<YearlyTick>,
) {
    // DAILY (always once per frame)
    daily.write(DailyTick);

    // MONTHLY (ONLY on transition)
    if game_time.month != last.month {
        monthly.write(MonthlyTick);
    }

    // YEARLY
    if game_time.year != last.year {
        yearly.write(YearlyTick);
    }

    // update tracker
    *last = LastGameTime {
        day: game_time.day,
        month: game_time.month,
        year: game_time.year,
    };
}