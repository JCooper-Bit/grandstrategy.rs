use bevy::prelude::*;
use crate::{LuaState, core::timing::GameTime, data::{DAYS_IN_MONTH, MONTHS_IN_YEAR}, lua_state::LuaCommand};


#[derive(Clone, Debug)]
pub struct ScheduledEvent {
    pub id: String,
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub callback: String,
}
#[derive(Resource, Default, Debug)]
pub struct Scheduler {
    pub events: Vec<ScheduledEvent>,
}

impl Scheduler {
    pub fn schedule(
        &mut self,
        id: String,
        day: u32,
        month: u32,
        year: u32,
        callback: String,
    ) {

        println!("[RUST API] Scheduling event: {} for {}/{}/{}", id, day, month, year);
        self.events.push(ScheduledEvent {
            id,
            day,
            month,
            year,
            callback: callback.to_string(),
        });
    }

    pub fn cancel(&mut self, id: String) {
        let pos = self.events.iter().position(|e| e.id == id);
        println!("[RUST API] Cancelling event: {}", id);
        if pos != None {
            self.events.remove(pos.unwrap());
        }
    }
    pub fn schedule_in_n_days(&mut self, id: String, n: u32, callback: String, t: &Res<GameTime>) {
        let day: u32;
        let month: u32;
        let year: u32;

        if t.day + n <= DAYS_IN_MONTH {
            day = t.day + n;
            month = t.month;
        }
        else {
            day = t.day + n - DAYS_IN_MONTH;
            // todo: Fix for month gap > 1 (generalize for x)
            month = t.month + 1;
        }

        if month > MONTHS_IN_YEAR {
            // todo: Fix for yea gap > 1 (generalize for x)
            year = t.year + 1
        }
        else {
            year = t.year
        }
        self.schedule(id, day, month, year, callback);
    }


}

pub fn process_schedule_commands(
    lua: NonSend<LuaState>,
    mut scheduler: ResMut<Scheduler>,
    t: Res<GameTime>
) {
    let mut commands = lua.schedule_commands.lock().unwrap();

    for cmd in commands.drain(..) {
        match cmd {
            LuaCommand::Schedule {id, day, month, year, callback } => {
                scheduler.schedule(id, day, month, year, callback);
            }

            LuaCommand::Cancel { id } => {
                scheduler.cancel(id);
            }
            LuaCommand::ScheduleInNDays {id, n, callback} => {
                scheduler.schedule_in_n_days(id, n, callback, &t)
            }
            
        }
    }
}



pub fn scheduler_system(
    mut scheduler: ResMut<Scheduler>,
    time: Res<GameTime>,
    lua: NonSend<LuaState>,
) {
    scheduler.events.retain(|event| {
        let due =
            event.year <= time.year &&
            event.month <= time.month &&
            event.day <= time.day;
        if due {
            println!("[Rust API] Firing Event: {}", event.callback);
            lua.call_game_function(&event.callback, (time.day, time.month, time.year));
            false
        } else {
            true
        }
    });
}