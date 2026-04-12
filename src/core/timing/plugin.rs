use bevy::prelude::*;

use crate::core::timing::*;

pub struct TimingPlugin;

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTime::default())
            .insert_resource(GameSpeed::VeryFast)
            .insert_resource(TickTimer {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            })
            .insert_resource(LastGameTime::default())
            .add_message::<TickMessage>()
            .add_message::<DailyTick>()
            .add_message::<MonthlyTick>()
            .add_message::<YearlyTick>()

            .add_systems(Update, (tick_driver, time_dispatcher))
            .add_systems(Update, print_game_date);
    }
}