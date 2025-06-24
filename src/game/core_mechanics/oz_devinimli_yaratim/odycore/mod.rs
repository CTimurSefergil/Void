<<<<<<< Updated upstream
use bevy::{
    app::{App, Startup, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Commands},
};

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    odycore::{
        building::{BuildingPropagationQueue, propagate_building_constraints},
        cell::{CellSpatialIndex, initialize_new_cells, update_spatial_index},
        open_space::{
            OpenSpacePropagationQueue, collapse_lowest_entropy_open_space_cell,
            propagate_open_space_constraints,
        },
    },
    odyrules::open_space_rules::OpenSpaceRules,
};

pub mod building;
pub mod cell;
pub mod open_space;

pub fn plugin(app: &mut App) {
    app.init_resource::<OpenSpaceRules>()
        .init_resource::<CellSpatialIndex>()
        .init_resource::<OpenSpacePropagationQueue>()
        .init_resource::<BuildingPropagationQueue>()
        .add_systems(Startup, setup_wfc_rules)
        .add_systems(
            Update,
            (
                update_spatial_index,
                initialize_new_cells,
                propagate_building_constraints,
                propagate_open_space_constraints,
                collapse_lowest_entropy_open_space_cell,
            )
                .chain(),
        );
}

fn setup_wfc_rules(mut commands: Commands) {
    commands.insert_resource(OpenSpaceRules::default());
    commands.insert_resource(CellSpatialIndex::default());
    commands.insert_resource(OpenSpacePropagationQueue::default());
    commands.insert_resource(BuildingPropagationQueue::default());
=======
use bevy::prelude::*;
pub mod open_space;

pub fn plugin(app: &mut App) {
    use open_space::{
        update_spatial_index,
        collapse_lowest_entropy_cell,
        propagate_newly_added_cells,
    };

    app
        .add_systems(Update, update_spatial_index)
        .add_systems(Update, collapse_lowest_entropy_cell)
        .add_systems(Update, propagate_newly_added_cells);
>>>>>>> Stashed changes
}
