use std::f32::consts::PI;

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::atmosphere::distance_fog;
use crate::collision::step;
use crate::house::Solids;

const EYE: f32 = 1.62;
const WALK: f32 = 2.1;
const SPRINT: f32 = 3.4;
const SENSITIVITY: f32 = 0.0022;

#[derive(Component, Default)]
struct Player {
    vel_y: f32,
    pitch: f32,
    speed: f32,
    bob: f32,
    lag: f32,
}

#[derive(Component)]
struct PlayerCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, lock_cursor))
            .add_systems(Update, (cursor_toggle, look, movement, camera_feel).chain());
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player::default(),
            Transform::from_xyz(3.6, 3.0, 7.6).with_rotation(Quat::from_rotation_y(PI)),
            Visibility::default(),
        ))
        .with_children(|p| {
            p.spawn((
                PlayerCamera,
                Camera3d::default(),
                Projection::Perspective(PerspectiveProjection {
                    fov: 72f32.to_radians(),
                    near: 0.05,
                    far: 80.0,
                    ..default()
                }),
                Transform::from_xyz(0.0, EYE, 0.0),
                distance_fog(),
            ));
        });
}

fn lock_cursor(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    options.grab_mode = CursorGrabMode::Locked;
    options.visible = false;
}

fn cursor_toggle(
    mut options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        options.grab_mode = CursorGrabMode::Locked;
        options.visible = false;
    }
    if keys.just_pressed(KeyCode::Escape) {
        options.grab_mode = CursorGrabMode::None;
        options.visible = true;
    }
}

fn look(
    motion: Res<AccumulatedMouseMotion>,
    options: Single<&CursorOptions, With<PrimaryWindow>>,
    mut player: Query<(&mut Transform, &mut Player)>,
) {
    if options.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let Ok((mut tf, mut p)) = player.single_mut() else {
        return;
    };
    tf.rotate_y(-motion.delta.x * SENSITIVITY);
    p.pitch = (p.pitch - motion.delta.y * SENSITIVITY).clamp(-1.45, 1.45);
}

fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    solids: Option<Res<Solids>>,
    mut player: Query<(&mut Transform, &mut Player)>,
) {
    let Some(solids) = solids else {
        return;
    };
    let Ok((mut tf, mut p)) = player.single_mut() else {
        return;
    };
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }

    let mut wish = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        wish.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        wish.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        wish.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        wish.x -= 1.0;
    }
    let wish = wish.normalize_or_zero();
    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        SPRINT
    } else {
        WALK
    };

    let forward = tf.rotation * Vec3::NEG_Z;
    let right = tf.rotation * Vec3::X;
    let delta = (forward * wish.y + right * wish.x) * speed * dt;

    let before = tf.translation;
    let (pos, vel_y, snapped) = step(before, p.vel_y, delta, dt, &solids.0);
    tf.translation = pos;
    p.vel_y = vel_y;
    p.lag -= snapped;

    let moved = Vec2::new(pos.x - before.x, pos.z - before.z).length() / dt;
    p.speed += (moved - p.speed) * (1.0 - (-10.0 * dt).exp());
}

fn camera_feel(
    time: Res<Time>,
    mut player: Query<&mut Player>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let (Ok(mut p), Ok(mut cam)) = (player.single_mut(), camera.single_mut()) else {
        return;
    };
    let dt = time.delta_secs();
    p.lag *= (-14.0 * dt).exp();
    p.bob += dt * p.speed * 2.6;
    let bob = p.bob.sin() * 0.022 * (p.speed / WALK).min(1.6);
    cam.translation.y = EYE + p.lag + bob;
    cam.rotation = Quat::from_rotation_x(p.pitch);
}
