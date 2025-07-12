pub mod common;
pub mod o_insan;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(o_insan::plugin);
}
