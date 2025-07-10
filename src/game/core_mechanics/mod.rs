use bevy::prelude::*;

pub mod enemy_ai;
pub mod movement;
pub mod oz_devinimli_yaratim;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);
    app.add_plugins(oz_devinimli_yaratim::plugin);
    app.add_plugins(enemy_ai::plugin);
}
