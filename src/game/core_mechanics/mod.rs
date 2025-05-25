use bevy::prelude::*;

pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);
}
