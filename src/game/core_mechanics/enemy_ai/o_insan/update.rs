use bevy::prelude::*;

use crate::game::{
    core_mechanics::enemy_ai::{
        common_events::{Depressed, PlayerInRange}, o_insan::{constants::PLAYER_SIGHT_DISTANCE, spawn::OInsan}
    },
    spawn::player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PlayerInRange>().add_event::<Depressed>().add_systems(Update, update_o_insan);
}

fn update_o_insan(
    player: Single<&Transform, With<Player>>,
    o_insan: Single<(Entity, &Transform), With<OInsan>>,
    mut commands: Commands
) {
    let (entity, enemy_transform) = o_insan.into_inner();
    if player.translation.distance(enemy_transform.translation) < PLAYER_SIGHT_DISTANCE {
        commands.trigger_targets(PlayerInRange {monster: entity, last_seen: enemy_transform.translation.x as i32}, entity);
    } else {
        commands.trigger_targets(Depressed, entity);
    }
}
