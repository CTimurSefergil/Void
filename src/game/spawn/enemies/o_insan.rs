use bevy::prelude::*;

use crate::game::spawn::enemies::common::Enemy;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemy);
}

#[derive(Component)]
struct OInsan;

fn spawn_enemy(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let ball_mesh = mesh_assets.add(Cuboid::default());
    let color = Color::srgb(0.05, 0.5, 0.6);
    let ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    commands
        .spawn((
            Transform::from_translation(Vec3 {
                x: 0.0,
                y: 24.0,
                z: 0.0,
            })
            .with_scale(Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            }),
            Mesh3d(ball_mesh),
            MeshMaterial3d(ball_material.clone()),
            Visibility::default(),
            Enemy,
            OInsan
        ))
       ;
}
