use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct HintText;

#[derive(Resource, Default)]
pub struct HintClock {
    pub locked_for: f32,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(HintClock::default());
    commands.spawn((
        Text::new("FPS"),
        TextFont {
            font_size: FontSize::Px(13.0),
            ..default()
        },
        TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: px(6),
            left: px(8),
            ..default()
        },
        FpsText,
    ));
    commands.spawn((
        Text::new("Click to look around  |  WASD walk  |  Shift run  |  F flashlight  |  F5 quality  |  Esc release mouse"),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgba(0.72, 0.72, 0.72, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(26),
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..default()
        },
        HintText,
    ));
}

pub fn update_fps(
    time: Res<Time>,
    quality: Res<crate::quality::Quality>,
    diagnostics: Res<DiagnosticsStore>,
    mut acc: Local<f32>,
    mut q: Query<&mut Text, With<FpsText>>,
) {
    *acc += time.delta_secs();
    if *acc < 0.25 && !quality.is_changed() {
        return;
    }
    *acc = 0.0;
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed());
    if let Some(fps) = fps {
        for mut t in &mut q {
            t.0 = format!("{fps:.0} FPS  [{}]", quality.label());
        }
    }
}

pub fn update_hint(
    time: Res<Time>,
    cursor: Single<&CursorOptions>,
    mut clock: ResMut<HintClock>,
    mut q: Query<&mut TextColor, With<HintText>>,
) {
    let locked = cursor.grab_mode == CursorGrabMode::Locked;
    if locked {
        clock.locked_for += time.delta_secs();
    }
    let alpha = if !locked {
        0.9
    } else {
        ((12.0 - clock.locked_for) / 3.0).clamp(0.0, 1.0) * 0.9
    };
    for mut c in &mut q {
        c.0 = Color::srgba(0.72, 0.72, 0.72, alpha);
    }
}
