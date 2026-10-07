use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use crate::util::hash2;

const SIZE: i32 = 256;

fn tile_noise(x: f32, y: f32, period: i32, seed: u32) -> f32 {
    let xi = x.floor();
    let yi = y.floor();
    let fx = x - xi;
    let fy = y - yi;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);
    let x0 = (xi as i32).rem_euclid(period);
    let y0 = (yi as i32).rem_euclid(period);
    let x1 = (x0 + 1).rem_euclid(period);
    let y1 = (y0 + 1).rem_euclid(period);
    let a = hash2(x0, y0, seed);
    let b = hash2(x1, y0, seed);
    let c = hash2(x0, y1, seed);
    let d = hash2(x1, y1, seed);
    let top = a + (b - a) * sx;
    let bot = c + (d - c) * sx;
    top + (bot - top) * sy
}

fn height(u: f32, v: f32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 0.5;
    let mut norm = 0.0;
    let mut period = 4;
    for o in 0..5u32 {
        sum += tile_noise(u * period as f32, v * period as f32, period, 11 + o * 7) * amp;
        norm += amp;
        amp *= 0.55;
        period *= 2;
    }
    sum / norm
}

fn mip_chain(base: Vec<u8>, size: usize) -> (Vec<u8>, u32) {
    let mut out = base.clone();
    let mut prev = base;
    let mut s = size;
    let mut levels = 1;
    while s > 1 {
        let n = s / 2;
        let mut next = vec![0u8; n * n * 4];
        for y in 0..n {
            for x in 0..n {
                for c in 0..4 {
                    let mut sum = 0u32;
                    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                        sum += prev[((y * 2 + dy) * s + x * 2 + dx) * 4 + c] as u32;
                    }
                    next[(y * n + x) * 4 + c] = (sum / 4) as u8;
                }
            }
        }
        out.extend_from_slice(&next);
        prev = next;
        s = n;
        levels += 1;
    }
    (out, levels)
}

fn make_image(data: Vec<u8>, format: TextureFormat) -> Image {
    let (full, levels) = mip_chain(data, SIZE as usize);
    let mut image = Image::default();
    image.texture_descriptor.size = Extent3d {
        width: SIZE as u32,
        height: SIZE as u32,
        depth_or_array_layers: 1,
    };
    image.texture_descriptor.format = format;
    image.texture_descriptor.mip_level_count = levels;
    image.data = full.into();
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    });
    image
}

pub fn make_detail_textures() -> (Image, Image) {
    let n = SIZE as usize;
    let mut h = vec![0.0_f32; n * n];
    for y in 0..n {
        for x in 0..n {
            h[y * n + x] = height(x as f32 / n as f32, y as f32 / n as f32);
        }
    }
    let at = |x: i32, y: i32| h[(y.rem_euclid(SIZE) as usize) * n + x.rem_euclid(SIZE) as usize];

    let mut albedo = Vec::with_capacity(n * n * 4);
    let mut normal = Vec::with_capacity(n * n * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let hv = at(x, y);
            let speck = (hash2(x, y, 901) - 0.5) * 0.1;
            let a = (0.85 + 0.5 * (hv - 0.5) + speck).clamp(0.0, 1.0);
            let g = (a * 255.0) as u8;
            albedo.extend_from_slice(&[g, g, g, 255]);

            let dx = at(x + 1, y) - at(x - 1, y);
            let dy = at(x, y + 1) - at(x, y - 1);
            let nv = Vec3::new(-dx * 3.0, -dy * 3.0, 1.0).normalize();
            normal.extend_from_slice(&[
                ((nv.x * 0.5 + 0.5) * 255.0) as u8,
                ((nv.y * 0.5 + 0.5) * 255.0) as u8,
                ((nv.z * 0.5 + 0.5) * 255.0) as u8,
                255,
            ]);
        }
    }
    (
        make_image(albedo, TextureFormat::Rgba8UnormSrgb),
        make_image(normal, TextureFormat::Rgba8Unorm),
    )
}
