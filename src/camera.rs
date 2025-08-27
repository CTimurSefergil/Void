use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, (update_camera, camera_look));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(DirectionalLight::default());

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Camera {
            ..Default::default()
        },
        IsDefaultUiCamera,     
        ClusterConfig::Single, 
    ));
}
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    let target_position = Vec3 {
        x: player.translation.x,
        y: player.translation.y + 3.0, 
        z: player.translation.z,
    };

    camera.translation = camera
        .translation
        .lerp(target_position, time.delta_secs() * 2.0);
}

fn camera_look(
    mut camera: Single<&mut Transform, With<IsDefaultUiCamera>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    let sensitivity = 1.0 * 100.0 / window.width().min(window.height());

    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(YXZ);

    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch -= mouse_motion.delta.y * dt * sensitivity;

    pitch = pitch.clamp(-1.57, 1.57); // Roughly -90° to +90°

    camera.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
}
