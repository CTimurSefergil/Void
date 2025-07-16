use bevy::prelude::*;

use crate::game::{
    core_mechanics::enemy_ai::{
        common_components::{PlayerInRange, Suspicious},
        o_insan::{constants::PLAYER_SIGHT_DISTANCE, spawn::OInsan},
    },
    spawn::player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_o_insan, hell_yeah));
}

fn update_o_insan(
    player: Single<&Transform, With<Player>>,
    o_insan: Single<(&mut Suspicious, &PlayerInRange, &Transform), With<OInsan>>,
) {
    let (mut suspicious, _player_in_range, enemy_transform) = o_insan.into_inner();
    if player.translation.distance(enemy_transform.translation) < PLAYER_SIGHT_DISTANCE {
        suspicious.0 = true;
    } else {
        suspicious.0 = false;
    }
}

fn hell_yeah(o_insan: Single<(&mut Suspicious, &PlayerInRange, &Transform), With<OInsan>>) {
    if o_insan.0.0 == true {
        println!("HELL YEAH");
    } else {
        println!("HELL NOT NOT YEEAAAAHH");
    }
}
