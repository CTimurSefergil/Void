use bevy::{
    app::{App, Startup, Update},
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
};

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    cells::CellSpatialIndex,
    odycore::open_space::{
        OpenSpacePropagationQueue, collapse_lowest_entropy_open_space_cell, initialize_new_cells,
        propagate_open_space_constraints, update_spatial_index,
    },
    odyrules::open_space_rules::OpenSpaceRules,
};

pub mod open_space;

pub fn plugin(app: &mut App) {
    app.init_resource::<OpenSpaceRules>()
        .init_resource::<OpenSpacePropagationQueue>()
        .add_systems(Startup, setup_wfc_rules)
        .add_systems(
            Update,
            (
                update_spatial_index,
                initialize_new_cells,
                propagate_open_space_constraints,
                collapse_lowest_entropy_open_space_cell.run_if(propagation_queue_empty),
            )
                .chain(),
        );
}

fn propagation_queue_empty(queue: Res<OpenSpacePropagationQueue>) -> bool {
    queue.queue.is_empty()
}

fn setup_wfc_rules(mut commands: Commands) {
    commands.insert_resource(OpenSpaceRules::default());
    commands.insert_resource(CellSpatialIndex::default());
    commands.insert_resource(OpenSpacePropagationQueue::default());
}
