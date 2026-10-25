use std::time::{Duration, Instant};

use bevy::prelude::*;

const STEPS: [Option<f32>; 5] = [Some(30.0), Some(60.0), Some(120.0), Some(144.0), None];

#[derive(Resource)]
pub struct FrameLimit {
    pub fps: Option<f32>,
    last: Instant,
}

impl FrameLimit {
    pub fn new(fps: Option<f32>) -> Self {
        Self {
            fps,
            last: Instant::now(),
        }
    }

    fn next(&self) -> Option<f32> {
        let i = STEPS.iter().position(|s| *s == self.fps).unwrap_or(0);
        STEPS[(i + 1) % STEPS.len()]
    }
}

pub fn keys(keys: Res<ButtonInput<KeyCode>>, mut limit: ResMut<FrameLimit>) {
    if keys.just_pressed(KeyCode::F6) {
        limit.fps = limit.next();
        match limit.fps {
            Some(f) => info!("fps limit = {f}"),
            None => info!("fps limit = off"),
        }
    }
}

pub fn wait(mut limit: ResMut<FrameLimit>) {
    let Some(fps) = limit.fps else {
        limit.last = Instant::now();
        return;
    };
    let deadline = limit.last + Duration::from_secs_f32(1.0 / fps);
    let now = Instant::now();
    if now >= deadline {
        limit.last = now;
        return;
    }
    let remain = deadline - now;
    if remain > Duration::from_millis(2) {
        std::thread::sleep(remain - Duration::from_millis(1));
    }
    while Instant::now() < deadline {
        std::hint::spin_loop();
    }
    limit.last = deadline;
}
