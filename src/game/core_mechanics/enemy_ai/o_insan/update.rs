use bevy::prelude::*;

use crate::game::{core_mechanics::enemy_ai::{common::{PlayerInRange, Suspicious}, o_insan::spawn::OInsan}, spawn::player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_o_insan);
}

fn update_o_insan(player: Single<&Transform, With<Player>>, o_insan: Single<(&mut Suspicious, &PlayerInRange, &Transform), With<OInsan>>) {
    let (mut suspicious, _player_in_range, enemy_transform) = o_insan.into_inner();
    if player.translation.distance(enemy_transform.translation) > 50.0 {
        suspicious.0 = true;
    }
    else {
        suspicious.0 = false;
    }
}