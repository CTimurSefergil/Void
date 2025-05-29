use bevy::prelude::*;

pub mod algoritma;
pub mod wfccore;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(algoritma::plugin);
}
