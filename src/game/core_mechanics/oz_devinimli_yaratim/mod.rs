use bevy::prelude::*;

pub mod cell_creation_destruction;
pub mod helper_functions;
pub mod odycore;
pub mod odyrules;
pub mod tiles_meshes_models;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        cell_creation_destruction::plugin,
        tiles_meshes_models::plugin,
        odycore::plugin,
    ));
}
