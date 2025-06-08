use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let ball_mesh = mesh_assets.add(Extrusion::new(Annulus::new(14.0, 15.0), 20.0));
    let color = Color::BLACK;
    let ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    commands
        .spawn((
            Transform::from_translation(Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            })
            .with_scale(Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            }),
            Player,
        ))
        .with_children(|parent| {
            parent.spawn((
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
                Mesh3d(ball_mesh.clone()),
                MeshMaterial3d(ball_material),
            ));
        });
}
