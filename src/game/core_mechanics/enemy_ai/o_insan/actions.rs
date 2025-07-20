use std::time::Duration;

use bevy::prelude::*;

use crate::game::core_mechanics::enemy_ai::o_insan::{
    constants::DEBUG_UPDATE_MS, memory::PlayerInRange,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SlowedDown>().add_observer(slow_down);
}

#[derive(Event)]
struct SlowedDown {
    monster: Entity,
}

fn slow_down(trigger: Trigger<PlayerInRange>, mut commands: Commands) {
    let PlayerInRange { monster, last_seen } = trigger.event();
    println!("{:?}, {:?}", monster, last_seen);
    commands.trigger(SlowedDown { monster: *monster });
}

fn debug_update(mut last_update: Local<Duration>, time: Res<Time>) -> bool {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(DEBUG_UPDATE_MS) > now {
        return false;
    }
    *last_update = now;
    true
}
