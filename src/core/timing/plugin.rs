use bevy::prelude::*;

use crate::core::timing::scheduler::{Scheduler, process_schedule_commands, scheduler_system};
use crate::core::timing::time::{GameTime, time_event_dispatcher};
use crate::core::timing::tick::{GameSpeed, TickTimer, tick_driver};
pub struct TimingPlugin;

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        app
            // ---- CORE TIME STATE ----
            .insert_resource(GameTime::default())
            .insert_resource(GameSpeed::Fast)
            .insert_resource(TickTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            })
            .insert_resource(Scheduler::default())
            // .insert_resource(LuaScheduleCommands::default())
            .add_systems(Update, tick_driver)

            .add_systems(Update, time_event_dispatcher)

            .add_systems(Update,(scheduler_system, process_schedule_commands));
    }
}