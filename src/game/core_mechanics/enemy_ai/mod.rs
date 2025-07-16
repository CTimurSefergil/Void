use bevy::prelude::*;

pub mod o_insan;
pub mod common;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(o_insan::plugin);
}
