use bevy::prelude::*;

pub mod spawn;
pub mod update;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, update::plugin));
}
