mod lua_state;
mod core;
mod data;
mod lua_events;
mod lua_systems;

use bevy::prelude::*;

use crate::lua_state::{LuaState};
use crate::core::{CorePlugin};
pub use crate::lua_events::{LuaEvent, LuaEventQueue};
use crate::lua_systems::lua_dispatch_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .insert_non_send_resource(LuaState::new())
        .insert_resource(LuaEventQueue::default())
        .add_systems(Update, lua_dispatch_system)
        .run();
}