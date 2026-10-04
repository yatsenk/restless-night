mod houses;
mod mesh;
mod props;
mod util;

use std::collections::{HashMap, VecDeque};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use avian3d::prelude::*;
use bevy::{
    core_pipeline::Skybox,
    input::mouse::AccumulatedMouseMotion,
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
        },
    },
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use houses::{Dims, build_house};
use mesh::MeshBuilder;
use props::*;
use util::*;

const FOG_RGB: [f32; 3] = [0.46, 0.48, 0.52];
const FOG_COLOR: Color = Color::srgb(FOG_RGB[0], FOG_RGB[1], FOG_RGB[2]);
const FOG_VISIBILITY: f32 = 15.0;
const FAR_PLANE: f32 = 15.0;

fn fog_density(visibility: f32) -> f32 {
    1.731 / visibility
}

const CHUNK: f32 = 80.0;
const ROAD_W: f32 = 10.0;
const CURB_W: f32 = 0.3;
const STRIP_W: f32 = 3.0;
const WALK_W: f32 = 2.4;
const WALK_IN: f32 = ROAD_W + CURB_W + STRIP_W;
const BLOCK_MIN: f32 = WALK_IN + WALK_W;
const BLOCK_MAX: f32 = CHUNK - BLOCK_MIN + ROAD_W;
const LOT: f32 = (BLOCK_MAX - BLOCK_MIN) * 0.5;
const FRONT: f32 = 7.2;
const FENCE_HALF: f32 = 14.0;
const FENCE_GAP: f32 = 1.9;

const LOAD_DIST: f32 = 22.0;
const UNLOAD_DIST: f32 = 34.0;

const LOD_HOUSE: f32 = 16.0;
const LOD_PROP: f32 = 8.0;
const LOD_HYST: f32 = 1.5;

const MOON_DIR: Vec3 = Vec3::new(-0.5, 0.38, -0.75);

const MOUSE_SENS: f32 = 0.0022;
const WALK_SPEED: f32 = 3.2;
const RUN_SPEED: f32 = 6.0;
const FLASHLIGHT_BASE: f32 = 350_000.0;
const FLASHLIGHT_SHADOWS: bool = false;

const LEAF_PALETTES: [([f32; 3], [f32; 3]); 4] = [
    ([0.50, 0.17, 0.03], [0.88, 0.46, 0.08]),
    ([0.42, 0.06, 0.04], [0.78, 0.20, 0.07]),
    ([0.55, 0.38, 0.05], [0.92, 0.72, 0.16]),
    ([0.33, 0.14, 0.05], [0.66, 0.33, 0.09]),
];

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Head;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Flashlight;

#[derive(Component)]
struct Leaf {
    speed: f32,
    phase: f32,
    spin: f32,
}

#[derive(Component)]
struct Lod {
    near: Handle<Mesh>,
    far: Handle<Mesh>,
    dist: f32,
    is_near: bool,
}

#[derive(Component)]
struct DistHide {
    dist: f32,
}

#[derive(Resource, Default)]
struct PlayerLook {
    yaw: f32,
    pitch: f32,
    bob: f32,
}

#[derive(Clone)]
struct LodMesh {
    near: Handle<Mesh>,
    far: Handle<Mesh>,
}

struct HouseVariant {
    body: LodMesh,
    glass: LodMesh,
    dims: Dims,
}

struct TreeVariant {
    mesh: LodMesh,
    trunk_r: f32,
    trunk_h: f32,
}

struct PumpkinVariant {
    mesh: LodMesh,
    face: Handle<Mesh>,
    radius: f32,
}

#[derive(Resource)]
struct CityAssets {
    vertex_mat: Handle<StandardMaterial>,
    path_mat: Handle<StandardMaterial>,
    window_lit: Handle<StandardMaterial>,
    window_dark: Handle<StandardMaterial>,
    pumpkin_mat: Handle<StandardMaterial>,
    face_mat: Handle<StandardMaterial>,
    lamp_glass_mat: Handle<StandardMaterial>,
    unit_cube: Handle<Mesh>,
    streets: Vec<Handle<Mesh>>,
    fence: LodMesh,
    graves: Vec<LodMesh>,
    bushes: Vec<LodMesh>,
    hay: LodMesh,
    leaf_piles: Vec<Handle<Mesh>>,
    ghost: Handle<Mesh>,
    lamp_pole: Handle<Mesh>,
    lamp_glass: Handle<Mesh>,
    mailbox: Handle<Mesh>,
    cars: Vec<LodMesh>,
    houses: Vec<HouseVariant>,
    trees: Vec<TreeVariant>,
    pumpkins: Vec<PumpkinVariant>,
}

#[derive(Resource, Default)]
struct Chunks {
    loaded: HashMap<IVec2, Vec<Entity>>,
    pending: VecDeque<(IVec2, u8)>,
}

fn hash2(x: i32, z: i32, s: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x27d4_eb2d)
        ^ (z as u32).wrapping_mul(0x1656_67b1)
        ^ s.wrapping_mul(0x9e37_79b1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    (h & 0xffff) as f32 / 65535.0
}

fn flat_quad(b: &mut MeshBuilder, x0: f32, z0: f32, x1: f32, z1: f32, y: f32, col: Rgba) {
    b.quad_out(
        Vec3::new(x0, y, z0),
        Vec3::new(x1, y, z0),
        Vec3::new(x1, y, z1),
        Vec3::new(x0, y, z1),
        Vec3::Y,
        col,
    );
}

fn build_street(variant: u32) -> Mesh {
    let mut b = MeshBuilder::default();
    let lawn_a = lin(0.09, 0.11, 0.05);
    let lawn_b = lin(0.20, 0.19, 0.08);
    let dirt = lin(0.17, 0.12, 0.07);
    let asphalt = lin(0.075, 0.075, 0.085);
    let curb = lin(0.34, 0.33, 0.31);
    let walk = lin(0.30, 0.29, 0.28);
    let walk_line = lin(0.18, 0.18, 0.18);
    let yellow = lin(0.80, 0.68, 0.20);
    let white = lin(0.78, 0.78, 0.76);

    let cell = 4.0;
    let n = (CHUNK / cell) as i32;
    for gx in 0..n {
        for gz in 0..n {
            let x0 = gx as f32 * cell;
            let z0 = gz as f32 * cell;
            if x0 + cell <= ROAD_W || z0 + cell <= ROAD_W {
                continue;
            }
            let v = hash2(gx, gz, variant);
            let col = if v > 0.88 {
                dirt
            } else {
                lerp_col(lawn_a, lawn_b, v)
            };
            flat_quad(&mut b, x0, z0, x0 + cell, z0 + cell, 0.0, col);
        }
    }

    let acell = 5.0;
    for i in 0..(CHUNK / acell) as i32 {
        for j in 0..2 {
            let v = hash2(i, j, variant + 11);
            let col = scale_col(asphalt, 0.9 + 0.2 * v);
            let x0 = i as f32 * acell;
            let z0 = j as f32 * acell;
            flat_quad(&mut b, x0, z0, x0 + acell, z0 + acell, 0.03, col);
        }
    }
    for i in 0..2 {
        for j in 2..(CHUNK / acell) as i32 {
            let v = hash2(i + 40, j, variant + 17);
            let col = scale_col(asphalt, 0.9 + 0.2 * v);
            let x0 = i as f32 * acell;
            let z0 = j as f32 * acell;
            flat_quad(&mut b, x0, z0, x0 + acell, z0 + acell, 0.03, col);
        }
    }

    let ch = 0.15;
    b.cuboid(
        Vec3::new(ROAD_W, 0.0, ROAD_W),
        Vec3::new(ROAD_W + CURB_W, ch, CHUNK),
        curb,
    );
    b.cuboid(
        Vec3::new(CHUNK - CURB_W, 0.0, ROAD_W),
        Vec3::new(CHUNK, ch, CHUNK),
        curb,
    );
    b.cuboid(
        Vec3::new(ROAD_W + CURB_W, 0.0, ROAD_W),
        Vec3::new(CHUNK - CURB_W, ch, ROAD_W + CURB_W),
        curb,
    );
    b.cuboid(
        Vec3::new(ROAD_W + CURB_W, 0.0, CHUNK - CURB_W),
        Vec3::new(CHUNK - CURB_W, ch, CHUNK),
        curb,
    );

    let wh = 0.12;
    let far_end = BLOCK_MAX + WALK_W;
    b.cuboid(
        Vec3::new(WALK_IN, 0.0, WALK_IN),
        Vec3::new(BLOCK_MIN, wh, far_end),
        walk,
    );
    b.cuboid(
        Vec3::new(BLOCK_MAX, 0.0, WALK_IN),
        Vec3::new(far_end, wh, far_end),
        walk,
    );
    b.cuboid(
        Vec3::new(BLOCK_MIN, 0.0, WALK_IN),
        Vec3::new(BLOCK_MAX, wh, BLOCK_MIN),
        walk,
    );
    b.cuboid(
        Vec3::new(BLOCK_MIN, 0.0, BLOCK_MAX),
        Vec3::new(BLOCK_MAX, wh, far_end),
        walk,
    );

    let mut t = WALK_IN + 2.0;
    while t < far_end {
        for (x0, x1) in [(WALK_IN, BLOCK_MIN), (BLOCK_MAX, far_end)] {
            b.cuboid(
                Vec3::new(x0, 0.0, t - 0.02),
                Vec3::new(x1, wh + 0.004, t + 0.02),
                walk_line,
            );
        }
        if t > BLOCK_MIN && t < BLOCK_MAX {
            for (z0, z1) in [(WALK_IN, BLOCK_MIN), (BLOCK_MAX, far_end)] {
                b.cuboid(
                    Vec3::new(t - 0.02, 0.0, z0),
                    Vec3::new(t + 0.02, wh + 0.004, z1),
                    walk_line,
                );
            }
        }
        t += 2.0;
    }

    for i in 0..9 {
        let c = 0.8 + i as f32;
        flat_quad(&mut b, c - 0.25, 10.5, c + 0.25, 12.7, 0.035, white);
        flat_quad(&mut b, 10.5, c - 0.25, 12.7, c + 0.25, 0.035, white);
    }
    let mut s = 14.0;
    while s + 3.0 <= CHUNK {
        flat_quad(&mut b, 4.9, s, 5.1, s + 3.0, 0.036, yellow);
        flat_quad(&mut b, s, 4.9, s + 3.0, 5.1, 0.036, yellow);
        s += 6.0;
    }
    for (x, z) in [(42.0_f32, 2.6_f32), (2.6, 55.0), (66.0, 7.4)] {
        b.disc(Vec3::new(x, 0.038, z), 0.5, 12, true, lin(0.12, 0.12, 0.13));
        b.disc(Vec3::new(x, 0.04, z), 0.38, 12, true, lin(0.18, 0.18, 0.19));
    }
    b.build()
}

fn lod_mesh(meshes: &mut Assets<Mesh>, near: Mesh, far: Mesh) -> LodMesh {
    LodMesh {
        near: meshes.add(near),
        far: meshes.add(far),
    }
}

fn build_assets(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> CityAssets {
    let vertex_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.92,
        ..default()
    });
    let path_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.24, 0.22),
        perceptual_roughness: 0.95,
        ..default()
    });
    let window_lit = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(4.0, 2.4, 0.7),
        ..default()
    });
    let window_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.04, 0.05, 0.07),
        perceptual_roughness: 0.15,
        ..default()
    });
    let pumpkin_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.55,
        ..default()
    });
    let face_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(5.0, 2.0, 0.2),
        ..default()
    });
    let lamp_glass_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::rgb(5.0, 3.6, 1.4),
        ..default()
    });

    let streets = (0..4)
        .map(|i| meshes.add(build_street(i * 7 + 3)))
        .collect();

    let fence = lod_mesh(
        meshes,
        build_fence(FENCE_HALF, FENCE_GAP, true),
        build_fence(FENCE_HALF, FENCE_GAP, false),
    );
    let graves = (0..3)
        .map(|k| {
            lod_mesh(
                meshes,
                build_gravestone(k, true),
                build_gravestone(k, false),
            )
        })
        .collect();
    let bushes = (0..3)
        .map(|k| {
            lod_mesh(
                meshes,
                build_bush(k as f32 * 3.1, true),
                build_bush(k as f32 * 3.1, false),
            )
        })
        .collect();
    let hay = lod_mesh(meshes, build_hay(true), build_hay(false));
    let leaf_piles = (0..2)
        .map(|k| meshes.add(build_leaf_pile(k as f32 * 2.3)))
        .collect();
    let ghost = meshes.add(build_ghost());
    let lamp_pole = meshes.add(build_lamp_pole());
    let lamp_glass = meshes.add(build_lamp_glass());
    let mailbox = meshes.add(build_mailbox());

    let car_colors = [
        [0.14, 0.20, 0.30],
        [0.35, 0.15, 0.08],
        [0.15, 0.25, 0.18],
        [0.55, 0.55, 0.50],
    ];
    let cars = car_colors
        .iter()
        .map(|c| lod_mesh(meshes, build_car(*c, true), build_car(*c, false)))
        .collect();

    let houses = (0..8)
        .map(|i| {
            let seed = 0xC17E + i as u64 * 7919;
            let (near, dims) = build_house(seed, true);
            let (far, _) = build_house(seed, false);
            HouseVariant {
                body: LodMesh {
                    near: meshes.add(near.body),
                    far: meshes.add(far.body),
                },
                glass: LodMesh {
                    near: meshes.add(near.glass),
                    far: meshes.add(far.glass),
                },
                dims,
            }
        })
        .collect();

    let trees = (0..4)
        .map(|i| {
            let seed = 0xB4E3 + i as u64 * 15_485_863;
            let (near, trunk_r, trunk_h) = build_tree(seed, true);
            let (far, _, _) = build_tree(seed, false);
            TreeVariant {
                mesh: lod_mesh(meshes, near, far),
                trunk_r,
                trunk_h,
            }
        })
        .collect();

    let pumpkin_specs: [(f32, f32, [f32; 3], [f32; 3]); 4] = [
        (0.30, 10.0, [0.55, 0.18, 0.02], [0.95, 0.45, 0.06]),
        (0.38, 12.0, [0.50, 0.16, 0.02], [0.92, 0.40, 0.05]),
        (0.46, 10.0, [0.45, 0.14, 0.02], [0.85, 0.34, 0.04]),
        (0.26, 9.0, [0.60, 0.32, 0.05], [0.95, 0.60, 0.12]),
    ];
    let pumpkins = pumpkin_specs
        .iter()
        .map(|(radius, ribs, dark, light)| {
            let d = lin(dark[0], dark[1], dark[2]);
            let l = lin(light[0], light[1], light[2]);
            PumpkinVariant {
                mesh: lod_mesh(
                    meshes,
                    build_pumpkin(*radius, *ribs, d, l, 14, 32),
                    build_pumpkin(*radius, *ribs, d, l, 5, 10),
                ),
                face: meshes.add(build_pumpkin_face(*radius)),
                radius: *radius,
            }
        })
        .collect();

    CityAssets {
        vertex_mat,
        path_mat,
        window_lit,
        window_dark,
        pumpkin_mat,
        face_mat,
        lamp_glass_mat,
        unit_cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        streets,
        fence,
        graves,
        bushes,
        hay,
        leaf_piles,
        ghost,
        lamp_pole,
        lamp_glass,
        mailbox,
        cars,
        houses,
        trees,
        pumpkins,
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(FOG_COLOR))
        .insert_resource(AmbientLight {
            color: Color::srgb(0.5, 0.55, 0.7),
            brightness: 120.0,
            ..default()
        })
        .insert_resource(GameRng(0x9E37_79B9_7F4A_7C15))
        .insert_resource(PlayerLook::default())
        .init_resource::<Chunks>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Restless Night".into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .add_systems(
            Startup,
            (setup_world, setup_player, setup_hud, grab_cursor_startup),
        )
        .add_systems(Update, (look_and_bob, player_move).chain())
        .add_systems(
            Update,
            (
                cursor_control,
                stream_chunks,
                update_lod,
                toggle_flashlight,
                flashlight_flicker,
                fog_breathing,
                animate_leaves,
            ),
        )
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GameRng>,
) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.55, 0.62, 0.9),
            illuminance: 2500.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(MOON_DIR.normalize() * 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20_000.0, 1.0, 20_000.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    let assets = build_assets(&mut meshes, &mut materials);
    commands.insert_resource(assets);

    let leaf_mats: Vec<Handle<StandardMaterial>> = LEAF_PALETTES
        .iter()
        .map(|(_, light)| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(light[0], light[1], light[2]),
                perceptual_roughness: 0.9,
                ..default()
            })
        })
        .collect();
    let leaf_mesh = meshes.add(Cuboid::new(0.16, 0.012, 0.11));
    for _ in 0..45 {
        let mat = leaf_mats[rng.pick(leaf_mats.len())].clone();
        commands.spawn((
            Leaf {
                speed: rng.range(0.5, 1.2),
                phase: rng.range(0.0, TAU),
                spin: rng.range(-2.0, 2.0),
            },
            Mesh3d(leaf_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(
                14.0 + rng.range(-8.0, 8.0),
                rng.range(0.5, 10.0),
                30.0 + rng.range(-8.0, 8.0),
            ),
        ));
    }
}

fn dist_to_chunk(p: Vec3, c: IVec2) -> f32 {
    let min = Vec2::new(c.x as f32 * CHUNK, c.y as f32 * CHUNK);
    let max = min + Vec2::splat(CHUNK);
    let pos = Vec2::new(p.x, p.z);
    pos.clamp(min, max).distance(pos)
}

fn stream_chunks(
    mut commands: Commands,
    assets: Res<CityAssets>,
    mut chunks: ResMut<Chunks>,
    player: Single<&Transform, With<Player>>,
) {
    let chunks = &mut *chunks;
    let p = player.translation;
    let center = IVec2::new((p.x / CHUNK).floor() as i32, (p.z / CHUNK).floor() as i32);

    let reach = (LOAD_DIST / CHUNK).floor() as i32 + 1;
    for dx in -reach..=reach {
        for dz in -reach..=reach {
            let c = center + IVec2::new(dx, dz);
            if dist_to_chunk(p, c) <= LOAD_DIST && !chunks.loaded.contains_key(&c) {
                chunks.loaded.insert(c, Vec::new());
                for stage in 0..5u8 {
                    chunks.pending.push_back((c, stage));
                }
            }
        }
    }

    let far: Vec<IVec2> = chunks
        .loaded
        .keys()
        .copied()
        .filter(|c| dist_to_chunk(p, *c) > UNLOAD_DIST)
        .collect();
    for c in far {
        if let Some(ents) = chunks.loaded.remove(&c) {
            for e in ents {
                commands.entity(e).despawn();
            }
        }
    }

    let mut budget = 2;
    while budget > 0 {
        let Some((c, stage)) = chunks.pending.pop_front() else {
            break;
        };
        let Some(list) = chunks.loaded.get_mut(&c) else {
            continue;
        };
        let ents = if stage == 0 {
            spawn_street(&mut commands, &assets, c)
        } else {
            spawn_lot(&mut commands, &assets, c, stage - 1)
        };
        list.extend(ents);
        budget -= 1;
    }
}

fn update_lod(
    time: Res<Time>,
    mut acc: Local<f32>,
    player: Single<&Transform, With<Player>>,
    mut lods: Query<(&GlobalTransform, &mut Lod, &mut Mesh3d)>,
    mut hides: Query<(&GlobalTransform, &DistHide, &mut Visibility)>,
) {
    *acc += time.delta_secs();
    if *acc < 0.12 {
        return;
    }
    *acc = 0.0;
    let p = player.translation;
    for (gt, mut lod, mut mesh) in &mut lods {
        let d = gt.translation().distance(p);
        if lod.is_near {
            if d > lod.dist + LOD_HYST {
                lod.is_near = false;
                mesh.0 = lod.far.clone();
            }
        } else if d < lod.dist {
            lod.is_near = true;
            mesh.0 = lod.near.clone();
        }
    }
    for (gt, h, mut vis) in &mut hides {
        let d = gt.translation().distance(p);
        let want = if d < h.dist {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != want {
            *vis = want;
        }
    }
}

fn lod_bundle(
    m: &LodMesh,
    mat: &Handle<StandardMaterial>,
    tf: Transform,
    dist: f32,
) -> (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform, Lod) {
    (
        Mesh3d(m.far.clone()),
        MeshMaterial3d(mat.clone()),
        tf,
        Lod {
            near: m.near.clone(),
            far: m.far.clone(),
            dist,
            is_near: false,
        },
    )
}

fn spawn_tree(
    commands: &mut Commands,
    a: &CityAssets,
    v: &TreeVariant,
    pos: Vec3,
    s: f32,
    yaw: f32,
) -> Entity {
    let ch = v.trunk_h * s;
    commands
        .spawn((
            Transform::from_xyz(pos.x, ch * 0.5, pos.z).with_rotation(Quat::from_rotation_y(yaw)),
            Visibility::default(),
            RigidBody::Static,
            Collider::cylinder(v.trunk_r * s, ch),
        ))
        .with_children(|p| {
            p.spawn(lod_bundle(
                &v.mesh,
                &a.vertex_mat,
                Transform::from_xyz(0.0, -ch * 0.5, 0.0).with_scale(Vec3::splat(s)),
                LOD_PROP + 4.0,
            ));
        })
        .id()
}

fn spawn_pumpkin(
    commands: &mut Commands,
    a: &CityAssets,
    idx: usize,
    face: bool,
    pos: Vec3,
    rot: Quat,
) -> Entity {
    let v = &a.pumpkins[idx];
    commands
        .spawn((
            lod_bundle(
                &v.mesh,
                &a.pumpkin_mat,
                Transform::from_translation(pos).with_rotation(rot),
                LOD_PROP,
            ),
            RigidBody::Dynamic,
            Collider::sphere(v.radius * 0.85),
            LinearDamping(0.8),
            AngularDamping(3.0),
        ))
        .with_children(|p| {
            if face {
                p.spawn((
                    Mesh3d(v.face.clone()),
                    MeshMaterial3d(a.face_mat.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                    DistHide { dist: 12.0 },
                ));
            }
        })
        .id()
}

fn spawn_lamp(commands: &mut Commands, a: &CityAssets, pos: Vec3) -> Entity {
    commands
        .spawn((
            Mesh3d(a.lamp_pole.clone()),
            MeshMaterial3d(a.vertex_mat.clone()),
            Transform::from_translation(pos),
            RigidBody::Static,
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(a.lamp_glass.clone()),
                MeshMaterial3d(a.lamp_glass_mat.clone()),
                Transform::default(),
            ));
            p.spawn((
                Collider::cylinder(0.25, 4.6),
                Transform::from_xyz(0.0, 2.3, 0.0),
            ));
        })
        .id()
}

fn spawn_car(commands: &mut Commands, a: &CityAssets, idx: usize, pos: Vec3, yaw: f32) -> Entity {
    commands
        .spawn((
            lod_bundle(
                &a.cars[idx],
                &a.vertex_mat,
                Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw)),
                LOD_PROP + 4.0,
            ),
            RigidBody::Static,
        ))
        .with_children(|p| {
            p.spawn((
                Collider::cuboid(4.5, 1.7, 1.9),
                Transform::from_xyz(0.0, 0.85, 0.0),
            ));
        })
        .id()
}

fn spawn_street(commands: &mut Commands, a: &CityAssets, c: IVec2) -> Vec<Entity> {
    let mut rng = GameRng::seeded(chunk_seed(c));
    let mut ents = Vec::new();
    let ox = c.x as f32 * CHUNK;
    let oz = c.y as f32 * CHUNK;

    let variant = ((hash2(c.x, c.y, 5) * 4.0) as usize).min(3);
    ents.push(
        commands
            .spawn((
                Mesh3d(a.streets[variant].clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                Transform::from_xyz(ox, 0.0, oz),
            ))
            .id(),
    );

    let near = ROAD_W + CURB_W + STRIP_W * 0.5;
    let far = CHUNK - CURB_W - STRIP_W * 0.5;

    for k in 0..2 {
        for edge in [near, far] {
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            ents.push(spawn_lamp(commands, a, Vec3::new(ox + edge, 0.0, oz + t)));
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            ents.push(spawn_lamp(commands, a, Vec3::new(ox + t, 0.0, oz + edge)));
        }
    }

    for k in 0..3 {
        for edge in [near, far] {
            if rng.chance(0.8) {
                let t = 18.0 + k as f32 * 26.0 + rng.range(-2.0, 2.0);
                let v = &a.trees[rng.pick(a.trees.len())];
                let s = rng.range(0.9, 1.25);
                let yaw = rng.range(0.0, TAU);
                ents.push(spawn_tree(
                    commands,
                    a,
                    v,
                    Vec3::new(ox + edge, 0.0, oz + t),
                    s,
                    yaw,
                ));
            }
            if rng.chance(0.8) {
                let t = 18.0 + k as f32 * 26.0 + rng.range(-2.0, 2.0);
                let v = &a.trees[rng.pick(a.trees.len())];
                let s = rng.range(0.9, 1.25);
                let yaw = rng.range(0.0, TAU);
                ents.push(spawn_tree(
                    commands,
                    a,
                    v,
                    Vec3::new(ox + t, 0.0, oz + edge),
                    s,
                    yaw,
                ));
            }
        }
    }

    if rng.chance(0.4) {
        let t = rng.range(20.0, 70.0);
        let lane = if rng.chance(0.5) { 2.5 } else { 7.5 };
        let yaw = if lane < 5.0 { 0.0 } else { PI } + rng.range(-0.1, 0.1);
        let idx = rng.pick(a.cars.len());
        ents.push(spawn_car(
            commands,
            a,
            idx,
            Vec3::new(ox + t, 0.03, oz + lane + rng.range(-0.5, 0.5)),
            yaw,
        ));
    }
    if rng.chance(0.4) {
        let t = rng.range(20.0, 70.0);
        let lane = if rng.chance(0.5) { 2.5 } else { 7.5 };
        let yaw = if lane < 5.0 { FRAC_PI_2 } else { -FRAC_PI_2 } + rng.range(-0.1, 0.1);
        let idx = rng.pick(a.cars.len());
        ents.push(spawn_car(
            commands,
            a,
            idx,
            Vec3::new(ox + lane + rng.range(-0.5, 0.5), 0.03, oz + t),
            yaw,
        ));
    }

    ents
}

struct StoneSpec {
    pos: Vec3,
    kind: usize,
    yaw: f32,
    tilt: f32,
}

struct PumpkinSpec {
    pos: Vec3,
    idx: usize,
    face: bool,
    yaw: f32,
}

fn spawn_lot(commands: &mut Commands, a: &CityAssets, c: IVec2, idx: u8) -> Vec<Entity> {
    let mut rng =
        GameRng::seeded(chunk_seed(c) ^ (idx as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let ix = (idx % 2) as f32;
    let iz = (idx / 2) as f32;
    let ox = c.x as f32 * CHUNK;
    let oz = c.y as f32 * CHUNK;
    let center = Vec3::new(
        ox + BLOCK_MIN + LOT * (ix + 0.5),
        0.0,
        oz + BLOCK_MIN + LOT * (iz + 0.5),
    );
    let yaw = if ix < 0.5 { 0.0 } else { PI };
    let rot = Quat::from_rotation_y(yaw);
    let world = |p: Vec3| center + rot * p;

    let v = &a.houses[rng.pick(a.houses.len())];
    let Dims { w, d, h, .. } = v.dims;
    let hx_c = -LOT * 0.5 + FRONT + d * 0.5;
    let x_front = hx_c - d * 0.5;
    let back_x = hx_c + d * 0.5 + 0.8;
    let win_mat = if rng.chance(0.55) {
        a.window_lit.clone()
    } else {
        a.window_dark.clone()
    };
    let front_edge = -LOT * 0.5;
    let step_x = x_front - 3.95;

    let mut stones = Vec::new();
    for _ in 0..rng.pick(6) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        stones.push(StoneSpec {
            pos: Vec3::new(
                rng.range(back_x + 1.0, 12.0),
                0.0,
                side * rng.range(1.5, 11.5),
            ),
            kind: rng.pick(a.graves.len()),
            yaw: rng.range(-0.5, 0.5),
            tilt: rng.range(-0.1, 0.1),
        });
    }

    let mut bushes = Vec::new();
    for sg in [-1.0_f32, 1.0] {
        bushes.push((
            Vec3::new(x_front - 0.8, 0.0, sg * (w * 0.5 - 0.9)),
            rng.pick(a.bushes.len()),
            rng.range(0.0, TAU),
        ));
    }
    for _ in 0..rng.pick(4) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        bushes.push((
            Vec3::new(rng.range(back_x, 12.8), 0.0, side * rng.range(2.0, 12.8)),
            rng.pick(a.bushes.len()),
            rng.range(0.0, TAU),
        ));
    }

    let mut pumpkins = vec![
        PumpkinSpec {
            pos: Vec3::new(x_front - 1.6, 1.0, 1.6),
            idx: 1,
            face: true,
            yaw: 0.0,
        },
        PumpkinSpec {
            pos: Vec3::new(x_front - 1.6, 0.9, -1.6),
            idx: 3,
            face: true,
            yaw: 0.0,
        },
    ];
    for _ in 0..(3 + rng.pick(4)) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        let face = rng.chance(0.35);
        pumpkins.push(PumpkinSpec {
            pos: Vec3::new(
                rng.range(-12.5, x_front - 4.5),
                0.0,
                side * rng.range(2.2, 12.5),
            ),
            idx: rng.pick(a.pumpkins.len()),
            face,
            yaw: if face {
                rng.range(-0.8, 0.8)
            } else {
                rng.range(0.0, TAU)
            },
        });
    }

    let hay = if rng.chance(0.4) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        Some((
            Vec3::new(
                rng.range(-11.0, x_front - 5.0),
                0.0,
                side * rng.range(8.0, 12.0),
            ),
            rng.range(0.0, TAU),
        ))
    } else {
        None
    };
    if let Some((p, _)) = hay {
        pumpkins.push(PumpkinSpec {
            pos: p + Vec3::new(0.2, 0.8 + a.pumpkins[2].radius, 0.0),
            idx: 2,
            face: rng.chance(0.5),
            yaw: rng.range(-0.8, 0.8),
        });
    }

    let ghost = if rng.chance(0.45) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        Some((
            Vec3::new(
                rng.range(back_x + 1.0, 11.0),
                0.0,
                side * rng.range(4.0, 11.0),
            ),
            rng.range(-0.6, 0.6),
        ))
    } else {
        None
    };

    let mut leaf_piles = Vec::new();
    for _ in 0..(2 + rng.pick(3)) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        let x = if rng.chance(0.5) {
            rng.range(-12.5, x_front - 4.5)
        } else {
            rng.range(back_x, 12.5)
        };
        leaf_piles.push((
            Vec3::new(x, 0.0, side * rng.range(2.0, 12.5)),
            rng.pick(a.leaf_piles.len()),
            rng.range(0.0, TAU),
            rng.range(0.7, 1.4),
        ));
    }

    let mut yard_trees = Vec::new();
    if rng.chance(0.7) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        yard_trees.push(Vec3::new(
            rng.range(-12.0, x_front - 5.0),
            0.0,
            side * rng.range(8.5, 12.5),
        ));
    }
    if rng.chance(0.5) {
        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
        yard_trees.push(Vec3::new(
            rng.range(back_x + 1.0, 12.0),
            0.0,
            side * rng.range(2.0, 12.0),
        ));
    }

    let path_len = step_x - front_edge + 0.1;
    let path_cx = (step_x + front_edge - 0.1) * 0.5;
    let hf = FENCE_HALF;
    let seg = hf - FENCE_GAP;
    let mid = (hf + FENCE_GAP) * 0.5;

    let mut ents = Vec::new();
    let root = commands
        .spawn((
            Transform::from_translation(center).with_rotation(rot),
            Visibility::default(),
            RigidBody::Static,
        ))
        .with_children(|p| {
            p.spawn(lod_bundle(
                &v.body,
                &a.vertex_mat,
                Transform::from_xyz(hx_c, 0.0, 0.0),
                LOD_HOUSE,
            ));
            p.spawn(lod_bundle(
                &v.glass,
                &win_mat,
                Transform::from_xyz(hx_c, 0.0, 0.0),
                LOD_HOUSE,
            ));
            p.spawn(lod_bundle(
                &a.fence,
                &a.vertex_mat,
                Transform::default(),
                LOD_HOUSE,
            ));

            p.spawn((
                Collider::cuboid(d, h, w),
                Transform::from_xyz(hx_c, h * 0.5, 0.0),
            ));
            p.spawn((
                Collider::cuboid(2.65, 0.55, 4.8),
                Transform::from_xyz(x_front - 1.3, 0.275, 0.0),
            ));
            for k in 0..2 {
                let x0 = x_front - 2.6 - 0.45 * (k + 1) as f32;
                let x1 = x_front - 2.55;
                let top = 0.55 - 0.18 * (k + 1) as f32;
                let zk = 1.2 + 0.06 * k as f32;
                p.spawn((
                    Collider::cuboid(x1 - x0, top, 2.0 * zk),
                    Transform::from_xyz((x0 + x1) * 0.5, top * 0.5, 0.0),
                ));
            }
            if let Some(zc) = v.dims.bay_z {
                p.spawn((
                    Collider::cuboid(1.0, houses::FLOOR_H, 2.5),
                    Transform::from_xyz(x_front - 0.5, houses::FLOOR_H * 0.5, zc),
                ));
            }
            for (size, pos) in [
                (Vec3::new(0.2, 1.4, seg), Vec3::new(-hf, 0.7, -mid)),
                (Vec3::new(0.2, 1.4, seg), Vec3::new(-hf, 0.7, mid)),
                (Vec3::new(0.2, 1.4, 2.0 * hf), Vec3::new(hf, 0.7, 0.0)),
                (Vec3::new(2.0 * hf, 1.4, 0.2), Vec3::new(0.0, 0.7, -hf)),
                (Vec3::new(2.0 * hf, 1.4, 0.2), Vec3::new(0.0, 0.7, hf)),
            ] {
                p.spawn((
                    Collider::cuboid(size.x, size.y, size.z),
                    Transform::from_translation(pos),
                ));
            }

            p.spawn((
                Mesh3d(a.unit_cube.clone()),
                MeshMaterial3d(a.path_mat.clone()),
                Transform::from_xyz(path_cx, 0.03, 0.0).with_scale(Vec3::new(path_len, 0.06, 1.8)),
            ));

            p.spawn((
                Mesh3d(a.mailbox.clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                Transform::from_xyz(front_edge + 0.9, 0.0, 2.6),
            ));
            p.spawn((
                Collider::cuboid(0.7, 1.4, 0.5),
                Transform::from_xyz(front_edge + 0.9, 0.7, 2.6),
            ));

            for s in &stones {
                let r = Quat::from_rotation_y(s.yaw) * Quat::from_rotation_z(s.tilt);
                p.spawn(lod_bundle(
                    &a.graves[s.kind],
                    &a.vertex_mat,
                    Transform::from_translation(s.pos).with_rotation(r),
                    LOD_PROP,
                ));
                let g = gravestone_collider(s.kind as u32);
                p.spawn((
                    Collider::cuboid(g.x, g.y, g.z),
                    Transform::from_translation(s.pos + Vec3::Y * (g.y * 0.5))
                        .with_rotation(Quat::from_rotation_y(s.yaw)),
                ));
            }

            for (pos, bi, byaw) in &bushes {
                p.spawn(lod_bundle(
                    &a.bushes[*bi],
                    &a.vertex_mat,
                    Transform::from_translation(*pos).with_rotation(Quat::from_rotation_y(*byaw)),
                    LOD_PROP,
                ));
                p.spawn((
                    Collider::sphere(0.75),
                    Transform::from_translation(*pos + Vec3::Y * 0.6),
                ));
            }

            if let Some((pos, hyaw)) = hay {
                p.spawn(lod_bundle(
                    &a.hay,
                    &a.vertex_mat,
                    Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(hyaw)),
                    LOD_PROP,
                ));
                p.spawn((
                    Collider::cuboid(1.2, 0.8, 0.9),
                    Transform::from_translation(pos + Vec3::Y * 0.4)
                        .with_rotation(Quat::from_rotation_y(hyaw)),
                ));
            }

            if let Some((pos, gyaw)) = ghost {
                p.spawn((
                    Mesh3d(a.ghost.clone()),
                    MeshMaterial3d(a.vertex_mat.clone()),
                    Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(gyaw)),
                ));
            }

            for (pos, li, lyaw, ls) in &leaf_piles {
                p.spawn((
                    Mesh3d(a.leaf_piles[*li].clone()),
                    MeshMaterial3d(a.vertex_mat.clone()),
                    Transform::from_translation(*pos)
                        .with_rotation(Quat::from_rotation_y(*lyaw))
                        .with_scale(Vec3::splat(*ls)),
                ));
            }
        })
        .id();
    ents.push(root);

    for pos in yard_trees {
        let v = &a.trees[rng.pick(a.trees.len())];
        let s = rng.range(0.9, 1.25);
        let tyaw = rng.range(0.0, TAU);
        ents.push(spawn_tree(commands, a, v, world(pos), s, tyaw));
    }

    for s in pumpkins {
        let r = a.pumpkins[s.idx].radius;
        let mut pos = s.pos;
        if pos.y < 0.01 {
            pos.y = r + 0.02;
        }
        ents.push(spawn_pumpkin(
            commands,
            a,
            s.idx,
            s.face,
            world(pos),
            rot * Quat::from_rotation_y(s.yaw),
        ));
    }

    ents
}

fn make_skybox_image() -> Image {
    const SIZE: u32 = 512;
    let moon_dir = MOON_DIR.normalize();
    let horizon = Vec3::new(FOG_RGB[0], FOG_RGB[1], FOG_RGB[2]);
    let zenith = Vec3::new(0.34, 0.36, 0.40);

    let mut data: Vec<u8> = Vec::with_capacity((SIZE * SIZE * 4 * 6) as usize);

    for face in 0..6 {
        for py in 0..SIZE {
            for px in 0..SIZE {
                let u = (px as f32 + 0.5) / SIZE as f32 * 2.0 - 1.0;
                let v = (py as f32 + 0.5) / SIZE as f32 * 2.0 - 1.0;
                let dir = match face {
                    0 => Vec3::new(1.0, -v, -u),
                    1 => Vec3::new(-1.0, -v, u),
                    2 => Vec3::new(u, 1.0, v),
                    3 => Vec3::new(u, -1.0, -v),
                    4 => Vec3::new(u, -v, 1.0),
                    _ => Vec3::new(-u, -v, -1.0),
                };
                let d = dir.normalize();

                let t = ((d.y - 0.75) / 0.25).clamp(0.0, 1.0);
                let t = t * t * (3.0 - 2.0 * t);
                let mut col = horizon.lerp(zenith, t);

                if d.y > 0.0 {
                    let cos_ang = d.dot(moon_dir);
                    let glow = ((cos_ang - 0.9) / 0.1).clamp(0.0, 1.0).powi(3) * 0.10;
                    col += Vec3::new(0.5, 0.55, 0.7) * glow;
                }

                data.extend_from_slice(&[
                    (col.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (col.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (col.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                ]);
            }
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    image
}

fn setup_player(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let skybox = images.add(make_skybox_image());

    commands
        .spawn((
            Player,
            Transform::from_xyz(14.5, 1.0, 30.0),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::capsule(0.35, 1.0),
            LockedAxes::ROTATION_LOCKED,
            Friction::ZERO,
            SleepingDisabled,
        ))
        .with_children(|p| {
            p.spawn((
                Head,
                Transform::from_xyz(0.0, 0.7, 0.0),
                Visibility::default(),
            ))
            .with_children(|h| {
                h.spawn((
                    PlayerCamera,
                    Camera3d::default(),
                    Camera {
                        clear_color: ClearColorConfig::Custom(FOG_COLOR),
                        ..default()
                    },
                    Msaa::Off,
                    Projection::Perspective(PerspectiveProjection {
                        fov: 1.2,
                        far: FAR_PLANE,
                        ..default()
                    }),
                    Transform::default(),
                    DistanceFog {
                        color: FOG_COLOR,
                        falloff: FogFalloff::Exponential {
                            density: FOG_VISIBILITY,
                        },
                        ..default()
                    },
                    Skybox {
                        image: skybox,
                        brightness: 1000.0,
                        ..default()
                    },
                ))
                .with_children(|c| {
                    c.spawn((
                        Flashlight,
                        SpotLight {
                            color: Color::srgb(1.0, 0.92, 0.75),
                            intensity: FLASHLIGHT_BASE,
                            range: 14.0,
                            radius: 0.04,
                            shadows_enabled: FLASHLIGHT_SHADOWS,
                            inner_angle: 0.12,
                            outer_angle: 0.55,
                            ..default()
                        },
                        Transform::from_xyz(0.2, -0.15, 0.0),
                    ));
                });
            });
        });
}

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("WASD move | Shift run | F flashlight | Esc release mouse | Click to capture"),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgba(0.8, 0.75, 0.65, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            bottom: Val::Px(10.0),
            ..default()
        },
    ));
}

fn grab(cursor: &mut CursorOptions) {
    cursor.grab_mode = if cfg!(target_os = "windows") {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::Locked
    };
    cursor.visible = false;
}

fn release(cursor: &mut CursorOptions) {
    cursor.grab_mode = CursorGrabMode::None;
    cursor.visible = true;
}

fn grab_cursor_startup(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    grab(&mut window.cursor_options);
}

fn cursor_control(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        release(&mut window.cursor_options);
    } else if mouse.just_pressed(MouseButton::Left) {
        grab(&mut window.cursor_options);
    }
}

fn look_and_bob(
    time: Res<Time>,
    mouse: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut look: ResMut<PlayerLook>,
    vel: Single<&LinearVelocity, With<Player>>,
    mut head: Single<&mut Transform, (With<Head>, Without<PlayerCamera>)>,
    mut cam: Single<&mut Transform, (With<PlayerCamera>, Without<Head>)>,
) {
    if window.cursor_options.grab_mode != CursorGrabMode::None {
        look.yaw -= mouse.delta.x * MOUSE_SENS;
        look.pitch = (look.pitch - mouse.delta.y * MOUSE_SENS).clamp(-1.5, 1.5);
    }

    let speed = Vec2::new(vel.0.x, vel.0.z).length();
    look.bob += speed * time.delta_secs() * 2.4;
    let amount = (speed / RUN_SPEED).min(1.0);

    head.rotation = Quat::from_rotation_y(look.yaw);
    cam.rotation = Quat::from_rotation_x(look.pitch);
    cam.translation.y = look.bob.sin() * 0.04 * amount;
}

fn player_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    look: Res<PlayerLook>,
    mut vel: Single<&mut LinearVelocity, With<Player>>,
) {
    let yaw_rot = Quat::from_rotation_y(look.yaw);
    let forward = yaw_rot * Vec3::NEG_Z;
    let right = yaw_rot * Vec3::X;

    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }

    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };
    let target = dir.normalize_or_zero() * speed;
    let current = Vec2::new(vel.0.x, vel.0.z);
    let blended = current.lerp(
        Vec2::new(target.x, target.z),
        (12.0 * time.delta_secs()).min(1.0),
    );
    vel.0.x = blended.x;
    vel.0.z = blended.y;
}

fn toggle_flashlight(
    keys: Res<ButtonInput<KeyCode>>,
    vis: Single<&mut Visibility, With<Flashlight>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let mut v = vis.into_inner();
        *v = if *v == Visibility::Hidden {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn flashlight_flicker(time: Res<Time>, light: Single<&mut SpotLight, With<Flashlight>>) {
    let t = time.elapsed_secs();
    let mut light = light.into_inner();
    let mut k = 0.92 + 0.08 * (t * 9.0).sin();
    if (t * 17.0).sin() * (t * 3.1).sin() > 0.97 {
        k *= 0.3;
    }
    light.intensity = FLASHLIGHT_BASE * k;
}

fn fog_breathing(time: Res<Time>, fog: Single<&mut DistanceFog, With<PlayerCamera>>) {
    let mut fog = fog.into_inner();
    let visibility = FOG_VISIBILITY + (time.elapsed_secs() * 0.15).sin();
    fog.falloff = FogFalloff::ExponentialSquared {
        density: fog_density(visibility),
    };
}

fn animate_leaves(
    time: Res<Time>,
    mut rng: ResMut<GameRng>,
    player: Single<&Transform, (With<Player>, Without<Leaf>)>,
    mut leaves: Query<(&mut Transform, &Leaf)>,
) {
    let dt = time.delta_secs();
    let t = time.elapsed_secs();
    let p = player.translation;
    for (mut tf, leaf) in &mut leaves {
        tf.translation.y -= leaf.speed * dt;
        tf.translation.x += (t * 0.8 + leaf.phase).sin() * 0.6 * dt;
        tf.translation.z += (t * 0.6 + leaf.phase * 1.7).cos() * 0.4 * dt;
        tf.rotate_local_x(leaf.spin * dt);
        tf.rotate_local_z(leaf.spin * 0.7 * dt);

        let flat = Vec2::new(tf.translation.x - p.x, tf.translation.z - p.z).length();
        if tf.translation.y < 0.05 || flat > 12.0 {
            tf.translation = Vec3::new(
                p.x + rng.range(-6.0, 6.0),
                rng.range(13.0, 16.0),
                p.z + rng.range(-6.0, 6.0),
            );
        }
    }
}
