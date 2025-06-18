use std::collections::VecDeque;

use bevy::ecs::{
    entity::Entity,
    resource::Resource,
    system::{Query, Res, ResMut},
};
use rand::seq::IteratorRandom;

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    helper_functions::get_random_tile::get_random_tile,
    odycore::cell::{Cell, CellSpatialIndex},
    odyrules::{building_rules::BuildingRules, open_space_rules::OpenSpaceRules},
};

#[derive(Resource, Default)]
pub struct BuildingPropagationQueue {
    pub queue: VecDeque<Entity>,
}

pub fn collapse_lowest_entropy_building_cell(
    mut building_queue: ResMut<BuildingPropagationQueue>,
    mut cells: Query<(Entity, &mut Cell)>,
    building_rules: Res<BuildingRules>,
) {
    // PSEUDO CODE for collapse_lowest_entropy_cell function:

    // 1. If propagation queue is not empty, return early (propagation has priority)
    // 2. Collect all uncollapsed cells as candidates
    // 3. If no candidates exist, return (all cells are collapsed)
    // 4. Find the minimum entropy value among all candidates
    // 5. Filter candidates to only include those with minimum entropy
    // 6. Randomly select one candidate from the filtered list
    // 7. If selected cell has valid tiles:
    //    a. Choose a random tile from valid tiles using rules
    //    b. Set cell as collapsed with chosen tile
    //    c. Reset entropy to 0
    //    d. Add entity to propagation queue for constraint propagation

    if !building_queue.queue.is_empty() {
        return;
    }

    let mut candidates = cells
        .iter_mut()
        .filter(|(_, cell)| !cell.is_collapsed)
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return;
    }

    let min_entropy = candidates
        .iter()
        .map(|(_, cell)| cell.entropy)
        .min()
        .unwrap_or(i32::MAX);

    candidates.retain(|(_, cell)| cell.entropy == min_entropy);

    if let Some((entity, cell)) = candidates
        .into_iter()
        .choose(&mut rand::rng())
        .map(|(e, c)| (e, c.into_inner()))
    {
        if !cell.valid_tiles.is_empty() {
            let tile = get_random_tile(building_rules.as_ref(), &cell.valid_tiles);
            cell.tile_type = Some(tile);
            cell.is_collapsed = true;
            cell.entropy = 0;

            building_queue.queue.push_back(entity);
        }
    }
}

pub fn propagate_building_constraints(
    mut wfc_queue: ResMut<BuildingPropagationQueue>,
    rules: Res<OpenSpaceRules>,
    spatial_index: Res<CellSpatialIndex>,
    mut cells: Query<&mut Cell>,
) {
}
