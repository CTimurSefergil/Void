use std::time::Duration;

use bevy::prelude::*;

use crate::game::core_mechanics::enemy_ai::{
    common_components::{PlayerInRange, Suspicious},
    o_insan::{constants::DEBUG_UPDATE_MS, spawn::OInsan},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (slow_down).run_if(debug_update));
}

fn slow_down(o_insan: Single<&PlayerInRange, With<OInsan>>) {
    if o_insan.0 == true {
        println!("Slowed Down");
    } else {
        println!("Normal Speed");
    }
}

fn debug_update(mut last_update: Local<Duration>, time: Res<Time>) -> bool {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(DEBUG_UPDATE_MS) > now {
        return false;
    }
    *last_update = now;
    true
}
