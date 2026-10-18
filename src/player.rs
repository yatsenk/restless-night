use bevy::{
    camera::{Exposure, Hdr},
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection},
    window::{CursorGrabMode, CursorOptions},
};

use crate::collision::Body;
use crate::math::v3;
use crate::world::HouseRes;

pub const EYE_HEIGHT: f32 = 1.62;
pub const WALK_SPEED: f32 = 2.3;
pub const RUN_SPEED: f32 = 4.0;
pub const LOOK_SENSITIVITY: f32 = 0.0022;
pub const START_EV: f32 = 5.4;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Flashlight;

#[derive(Resource)]
pub struct PlayerState {
    pub body: Body,
    pub yaw: f32,
    pub pitch: f32,
    pub vel: Vec2,
    pub eye_y: f32,
    pub bob: f32,
    pub flashlight: bool,
}

pub fn setup(mut commands: Commands, house: Res<HouseRes>) {
    let s = house.0.spawn;
    commands.insert_resource(PlayerState {
        body: Body::new(v3(s.x, s.y, s.z)),
        yaw: house.0.spawn_yaw,
        pitch: 0.0,
        vel: Vec2::ZERO,
        eye_y: s.y + EYE_HEIGHT,
        bob: 0.0,
        flashlight: true,
    });

    commands
        .spawn((
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                fov: 72.0_f32.to_radians(),
                near: 0.05,
                far: 70.0,
                ..default()
            }),
            Transform::from_xyz(s.x, s.y + EYE_HEIGHT, s.z),
            PlayerCamera,
            Tonemapping::TonyMcMapface,
            Exposure { ev100: START_EV },
            Hdr,
        ))
        .insert((
            ColorGrading {
                global: ColorGradingGlobal {
                    post_saturation: 0.72,
                    temperature: -0.06,
                    ..default()
                },
                midtones: ColorGradingSection {
                    contrast: 1.08,
                    ..default()
                },
                ..default()
            },
            DistanceFog {
                color: crate::atmosphere::haze_color(),
                falloff: FogFalloff::ExponentialSquared {
                    density: crate::atmosphere::FOG_DENSITY,
                },
                ..default()
            },
            Msaa::Off,
        ))
        .with_children(|p| {
            p.spawn((
                SpotLight {
                    color: Color::srgb(1.0, 0.93, 0.78),
                    intensity: 4200.0,
                    range: 16.0,
                    inner_angle: 0.13,
                    outer_angle: 0.42,
                    shadow_maps_enabled: true,
                    ..default()
                },
                Transform::default(),
                Flashlight,
            ));
        });
}

pub fn cursor_grab(
    mut cursor: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn look(
    cursor: Single<&CursorOptions>,
    motion: Res<AccumulatedMouseMotion>,
    mut st: ResMut<PlayerState>,
) {
    if cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }
    st.yaw -= motion.delta.x * LOOK_SENSITIVITY;
    st.pitch = (st.pitch - motion.delta.y * LOOK_SENSITIVITY).clamp(-1.45, 1.45);
}

pub fn walk(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    house: Res<HouseRes>,
    mut st: ResMut<PlayerState>,
) {
    let dt = time.delta_secs().min(0.05);
    if dt <= 0.0 {
        return;
    }
    let mut input = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        input.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        input.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        input.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        input.x -= 1.0;
    }
    let speed = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };
    let forward = Vec2::new(-st.yaw.sin(), -st.yaw.cos());
    let right = Vec2::new(st.yaw.cos(), -st.yaw.sin());
    let wish = (forward * input.y + right * input.x).normalize_or_zero() * speed;
    let k = 1.0 - (-14.0 * dt).exp();
    st.vel = st.vel.lerp(wish, k);
    let d = st.vel * dt;
    house.0.world.step(&mut st.body, d.x, d.y, dt);
    if st.body.pos.y < -20.0 {
        let s = house.0.spawn;
        st.body = Body::new(v3(s.x, s.y, s.z));
        st.vel = Vec2::ZERO;
    }
    let moving = st.vel.length();
    if st.body.grounded {
        st.bob += moving * dt * 2.1;
    }
    let target = st.body.pos.y + EYE_HEIGHT;
    let diff = target - st.eye_y;
    if diff.abs() > 0.7 {
        st.eye_y = target;
    } else {
        st.eye_y += diff * (1.0 - (-20.0 * dt).exp());
    }
}

pub fn apply_camera(
    st: Res<PlayerState>,
    mut cam: Single<&mut Transform, With<PlayerCamera>>,
) {
    let amp = (st.vel.length() / RUN_SPEED).min(1.0);
    let bob_y = (st.bob * 2.0).sin() * 0.011 * amp;
    let bob_x = st.bob.sin() * 0.007 * amp;
    let rot = Quat::from_euler(EulerRot::YXZ, st.yaw, st.pitch, 0.0);
    cam.translation = Vec3::new(st.body.pos.x, st.eye_y + bob_y, st.body.pos.z) + rot * Vec3::new(bob_x, 0.0, 0.0);
    cam.rotation = rot;
}

pub fn flashlight(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut st: ResMut<PlayerState>,
    mut q: Query<(&mut SpotLight, &mut Visibility), With<Flashlight>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        st.flashlight = !st.flashlight;
    }
    let t = time.elapsed_secs();
    let dip = {
        let slot = (t * 9.0).floor();
        let h = ((slot * 12.9898).sin() * 43758.547).fract().abs();
        if h > 0.985 { 0.35 } else { 1.0 }
    };
    for (mut l, mut v) in &mut q {
        l.intensity = 4200.0 * dip;
        *v = if st.flashlight { Visibility::Inherited } else { Visibility::Hidden };
    }
}
