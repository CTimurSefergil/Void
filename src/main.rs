use bevy::prelude::*;

mod camera;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::plugin)
        .add_plugins(game::plugin)
        .run();
}
