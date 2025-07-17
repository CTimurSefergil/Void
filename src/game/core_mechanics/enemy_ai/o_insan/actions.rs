use std::time::Duration;

use bevy::prelude::*;

use crate::game::core_mechanics::enemy_ai::{
    common_events::PlayerInRange, o_insan::constants::DEBUG_UPDATE_MS
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (slow_down).run_if(debug_update));
}

fn slow_down(mut events: EventReader<PlayerInRange>) {
    for PlayerInRange {monster, last_seen} in events.read() {
        println!("{:?}, {:?}", monster, last_seen);
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
