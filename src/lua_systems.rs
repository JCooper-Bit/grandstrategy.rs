use bevy::prelude::*;
use crate::{LuaState, LuaEventQueue, LuaEvent};

pub fn lua_dispatch_system(
    lua: NonSend<LuaState>,
    mut queue: ResMut<LuaEventQueue>,
) {
    for event in queue.events.drain(..) {
        match event {
            LuaEvent::MonthTick { day, month, year } => {
                lua.call_game_function("on_month_tick", (day, month, year));
            }

            LuaEvent::DayTick { day, month, year } => {
                lua.call_game_function("on_day_tick", (day, month, year));
            }
            LuaEvent::YearTick { day, month, year } => {
                lua.call_game_function("on_year_tick", (day, month, year));
            }
        }
    }
}