pub mod core_mechanics;
pub mod spawn;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((spawn::plugin, core_mechanics::plugin));
}
