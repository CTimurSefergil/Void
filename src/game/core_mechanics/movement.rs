use bevy::{
    input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion},
    prelude::*,
    render::camera,
    window::PrimaryWindow,
};

use crate::game::spawn::player::Player;

const MOVEMENT_SPEED: f32 = 31.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_movement, //.after(player_look),
            //player_look,
            //camera_look,
            focus_event,
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
        ),
    )
    .add_observer(apply_grab);
}

#[derive(Event, Deref)]
struct GrabEvent(bool);

fn player_movement(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut intent = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.z += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // AŞAĞIDAKİ 4 SATIR OLMADIĞI MÜDDETÇE BAKTIĞI YÖNE DOĞRU GİTMİYOR, NEDEN ???
    let forward = player.forward().as_vec3() * intent.z;
    let right = player.right().as_vec3() * intent.x;
    let mut to_move = forward + right;
    to_move.y = 0.0;

    player.translation += to_move.normalize_or_zero() * time.delta_secs() * MOVEMENT_SPEED;
}

fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
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
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    pitch = pitch.clamp(-1.57, 1.57);

    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
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
    pitch = pitch.clamp(-1.57, 1.57);

    camera.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
}

fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    use bevy::window::CursorGrabMode;
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

use bevy::window::WindowFocused;
fn focus_event(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}
