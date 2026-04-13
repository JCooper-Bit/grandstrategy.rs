use bevy::prelude::*;
use crate::{LuaEvent, LuaEventQueue, data::{DAYS_IN_MONTH, MONTHS_IN_YEAR, STARTING_YEAR}};

#[derive(Resource)]
pub struct GameTime {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: STARTING_YEAR,
        }
    }
}

impl GameTime {
    pub fn advance_day(&mut self) {
        self.day += 1;

        if self.day > DAYS_IN_MONTH {
            self.day = 1;
            self.month += 1;
        }

        if self.month > MONTHS_IN_YEAR {
            self.month = 1;
            self.year += 1;
        }
    }
}

/// Snapshot used for clean transition detection
#[derive(Clone, Copy)]
pub struct GameTimeSnapshot {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl Default for GameTimeSnapshot {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: STARTING_YEAR,
        }
    }
}

/// Converts GameTime changes into Lua events
pub fn time_event_dispatcher(
    game_time: Res<GameTime>,
    mut queue: ResMut<LuaEventQueue>,
    mut last: Local<GameTimeSnapshot>,
) {
    // DAY tick (always when changed)
    if game_time.day != last.day {
        queue.events.push(LuaEvent::DayTick {
            day: game_time.day,
            month: game_time.month,
            year: game_time.year,
        });
    }

    // MONTH tick
    if game_time.month != last.month {
        queue.events.push(LuaEvent::MonthTick {
            day: game_time.day,
            month: game_time.month,
            year: game_time.year,
        });
    }

    if game_time.year != last.year {
        queue.events.push(LuaEvent::YearTick {
            day: game_time.day,
            month: game_time.month,
            year: game_time.year,
        });    }
    // update snapshot
    *last = GameTimeSnapshot {
        day: game_time.day,
        month: game_time.month,
        year: game_time.year,
    };
}