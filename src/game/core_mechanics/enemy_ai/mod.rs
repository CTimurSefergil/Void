use bevy::prelude::*;

pub mod common_components;
pub mod o_insan;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(o_insan::plugin);
}
