use bevy::prelude::*;

pub mod algoritma;
pub mod odycore;
pub mod odyrules;
pub mod tiles_meshes_models;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        algoritma::plugin,
        tiles_meshes_models::plugin,
        odycore::plugin,
    ));
}
