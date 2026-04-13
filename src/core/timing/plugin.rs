use bevy::prelude::*;

use crate::core::timing::time::{GameTime, time_event_dispatcher};
use crate::core::timing::tick::{GameSpeed, TickTimer, tick_driver};
pub struct TimingPlugin;

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        app
            // ---- CORE TIME STATE ----
            .insert_resource(GameTime::default())
            .insert_resource(GameSpeed::VeryFast)
            .insert_resource(TickTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            })

            // ---- SYSTEMS ----
            // 1. advances time
            .add_systems(Update, tick_driver)

            // 2. converts time changes → Lua events
            .add_systems(Update, time_event_dispatcher);
    }
}