use bevy::prelude::*;

use crate::game::{
    core_mechanics::enemy_ai::{
        common_components::{PlayerInRange, Suspicious},
        o_insan::{constants::PLAYER_SIGHT_DISTANCE, spawn::OInsan},
    },
    spawn::player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_o_insan);
}

fn update_o_insan(
    player: Single<&Transform, With<Player>>,
    o_insan: Single<(&mut Suspicious, &mut PlayerInRange, &Transform), With<OInsan>>,
) {
    let (mut _suspicious, mut player_in_range, enemy_transform) = o_insan.into_inner();
    if player.translation.distance(enemy_transform.translation) < PLAYER_SIGHT_DISTANCE {
        player_in_range.0 = true;
    } else {
        player_in_range.0 = false;
    }
}
