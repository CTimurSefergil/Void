use bevy::prelude::*;

pub mod algoritma;
pub mod tiles_meshes_models;
pub mod wfccore;
pub mod wfcrules;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        algoritma::plugin,
        tiles_meshes_models::plugin,
        wfccore::plugin,
    ));
}
