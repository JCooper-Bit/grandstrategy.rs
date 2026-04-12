
use crate::{core::DailyTick, data::{DAYS_IN_MONTH, MONTHS_IN_YEAR, STARTING_YEAR}};
use bevy::prelude::*;



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
            year: STARTING_YEAR
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

pub fn print_game_date(mut events: MessageReader<DailyTick>,
time: Res<GameTime>, ) {
    for _event in events.read() {
        println!(
            "Game date: {:02}-{:02}-{}",
            time.day, time.month, time.year
        );
    }
}