use bevy::prelude::*;

pub mod o_insan;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(o_insan::SimpleAIPlugin); 
}
