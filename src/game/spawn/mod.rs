pub mod player;
pub mod enemies;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, enemies::plugin));
}
