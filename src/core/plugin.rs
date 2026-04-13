use bevy::prelude::*;

use crate::core::timing::TimingPlugin;
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TimingPlugin);
    
    }
}