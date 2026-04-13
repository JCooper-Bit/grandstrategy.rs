use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum LuaEvent {
    DayTick { day: u32, month: u32, year: u32 },
    MonthTick { day: u32, month: u32, year: u32 },
    YearTick { day: u32, month: u32, year: u32 },

}

#[derive(Resource, Default)]
pub struct LuaEventQueue {
    pub events: Vec<LuaEvent>,
}