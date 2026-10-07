mod clutter;
mod far;
mod ground;
mod houses;
mod mesh;
mod props;
mod texture;
mod util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::f32::consts::{PI, TAU};

use avian3d::prelude::*;
use bevy::{
    core_pipeline::{
        Skybox,
        bloom::Bloom,
        smaa::{Smaa, SmaaPreset},
        tonemapping::Tonemapping,
    },
    input::mouse::AccumulatedMouseMotion,
    pbr::{
        CascadeShadowConfigBuilder, DistanceFog, FogFalloff, FogVolume, NotShadowCaster,
        ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel, VolumetricFog,
        VolumetricLight,
    },
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
        },
        view::{ColorGrading, ColorGradingGlobal, ColorGradingSection},
    },
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use houses::{Dims, build_house};
use props::*;
use util::*;

const HAZE_RGB: [f32; 3] = [0.034, 0.046, 0.072];
const ZENITH_RGB: [f32; 3] = [0.006, 0.010, 0.026];
const HAZE_COLOR: Color = Color::srgb(HAZE_RGB[0], HAZE_RGB[1], HAZE_RGB[2]);
const HAZE_START: f32 = 120.0;
const HAZE_END: f32 = 310.0;
const FAR_PLANE: f32 = 340.0;

const MIST_DENSITY: f32 = 0.03;
const MIST_STEPS: u32 = 40;
const MIST_SIZE: Vec3 = Vec3::new(110.0, 14.0, 110.0);

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

const LOAD_DIST: f32 = 130.0;
const UNLOAD_DIST: f32 = 155.0;
const FAR_LOAD_DIST: f32 = 300.0;
const FAR_UNLOAD_DIST: f32 = 340.0;
const FAR_BUDGET: usize = 3;

const LOD_HOUSE: f32 = 55.0;
const LOD_TREE: f32 = 95.0;
const LOD_PROP: f32 = 22.0;
const LOD_HYST: f32 = 3.0;
const GROUND_FULL: f32 = 55.0;
const LITTER_DIST: f32 = 48.0;
const DETAIL_DIST: f32 = 50.0;
const PROP_CULL: f32 = 95.0;

const MOON_DIR: Vec3 = Vec3::new(-0.5, 0.38, -0.75);

const MOUSE_SENS: f32 = 0.0022;
const WALK_SPEED: f32 = 3.2;
const RUN_SPEED: f32 = 6.0;
const FLASHLIGHT_BASE: f32 = 300_000.0;
const MOON_LUX: f32 = 800.0;
const MOON_SHADOWS: bool = true;
const MOON_RADIUS_DEG: f32 = 3.0;
const AMBIENT_LEVEL: f32 = 45.0;
const LAMP_LUMENS: f32 = 300_000.0;
const LAMP_LIT_CHANCE: f32 = 0.6;
const WINDOW_LIT_CHANCE: f32 = 0.08;
const FLASHLIGHT_SHADOWS: bool = true;
const FLASHLIGHT_BEAM: bool = false;
const SSAO_ENABLED: bool = false;

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
struct StatsText;

#[derive(Component)]
struct Mist;

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
struct GroundTier {
    min: Vec2,
    max: Vec2,
    from: f32,
    to: f32,
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

struct StreetMeshes {
    full: Handle<Mesh>,
    lite: Handle<Mesh>,
    litter: Vec<Handle<Mesh>>,
}

struct HouseVariant {
    body: LodMesh,
    glass: LodMesh,
    detail: Handle<Mesh>,
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
    far_glow_mat: Handle<StandardMaterial>,
    unit_cube: Handle<Mesh>,
    streets: Vec<StreetMeshes>,
    graves: Vec<LodMesh>,
    bushes: Vec<LodMesh>,
    hay: LodMesh,
    leaf_piles: Vec<Handle<Mesh>>,
    ghost: Handle<Mesh>,
    lamp_pole: Handle<Mesh>,
    lamp_glass: Handle<Mesh>,
    mailbox: Handle<Mesh>,
    houses: Vec<HouseVariant>,
    trees: Vec<TreeVariant>,
    pumpkins: Vec<PumpkinVariant>,
}

#[derive(Resource, Default)]
struct Chunks {
    loaded: HashMap<IVec2, Vec<Entity>>,
    pending: VecDeque<(IVec2, u8)>,
    far: HashMap<IVec2, Vec<Entity>>,
    far_hidden: HashSet<IVec2>,
}

fn lod_mesh(meshes: &mut Assets<Mesh>, near: Mesh, far: Mesh) -> LodMesh {
    LodMesh {
        near: meshes.add(near),
        far: meshes.add(far),
    }
}

fn build_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> CityAssets {
    let (detail_albedo, detail_normal) = texture::make_detail_textures();
    let vertex_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(images.add(detail_albedo)),
        normal_map_texture: Some(images.add(detail_normal)),
        perceptual_roughness: 0.9,
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
    let far_glow_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.72, 0.30),
        unlit: true,
        ..default()
    });

    let streets = (0..4)
        .map(|i| {
            let seed = i * 7 + 3;
            StreetMeshes {
                full: meshes.add(ground::build_street(seed, false)),
                lite: meshes.add(ground::build_street(seed, true)),
                litter: ground::build_litter(seed)
                    .into_iter()
                    .map(|m| meshes.add(m))
                    .collect(),
            }
        })
        .collect();

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
                detail: meshes.add(clutter::build_house_detail(seed, dims)),
                dims,
            }
        })
        .collect();

    let trees = (0..7)
        .map(|i| {
            let leafy = i >= 4;
            let seed = if leafy {
                0xCAFE + (i - 4) as u64 * 104_729
            } else {
                0xB4E3 + i as u64 * 15_485_863
            };
            let (near, trunk_r, trunk_h) = build_tree(seed, true, leafy);
            let (far, _, _) = build_tree(seed, false, leafy);
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
        far_glow_mat,
        unit_cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        streets,
        graves,
        bushes,
        hay,
        leaf_piles,
        ghost,
        lamp_pole,
        lamp_glass,
        mailbox,
        houses,
        trees,
        pumpkins,
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(HAZE_COLOR))
        .insert_resource(AmbientLight {
            color: Color::srgb(0.35, 0.42, 0.75),
            brightness: AMBIENT_LEVEL,
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
                update_stats,
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
    mut images: ResMut<Assets<Image>>,
    mut rng: ResMut<GameRng>,
) {
    let mut moon = commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.55, 0.65, 1.0),
            illuminance: MOON_LUX,
            shadows_enabled: MOON_SHADOWS,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 2,
            minimum_distance: 0.1,
            maximum_distance: 60.0,
            first_cascade_far_bound: 14.0,
            overlap_proportion: 0.2,
        }
        .build(),
        Transform::from_translation(MOON_DIR.normalize() * 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    if MOON_SHADOWS {
        moon.insert(VolumetricLight);
    }

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20_000.0, 1.0, 20_000.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    let assets = build_assets(&mut meshes, &mut materials, &mut images);
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
    mut meshes: ResMut<Assets<Mesh>>,
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

    let gone: Vec<IVec2> = chunks
        .loaded
        .keys()
        .copied()
        .filter(|c| dist_to_chunk(p, *c) > UNLOAD_DIST)
        .collect();
    for c in gone {
        if let Some(ents) = chunks.loaded.remove(&c) {
            for e in ents {
                commands.entity(e).despawn();
            }
        }
    }

    let mut budget = 2;
    while budget > 0 {
        let Some(best) = chunks
            .pending
            .iter()
            .enumerate()
            .min_by(|a, b| dist_to_chunk(p, a.1.0).total_cmp(&dist_to_chunk(p, b.1.0)))
            .map(|(i, _)| i)
        else {
            break;
        };
        let Some((c, stage)) = chunks.pending.remove(best) else {
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

    let far_reach = (FAR_LOAD_DIST / CHUNK).floor() as i32 + 1;
    let mut far_budget = FAR_BUDGET;
    let mut wanted: Vec<(f32, IVec2)> = Vec::new();
    for dx in -far_reach..=far_reach {
        for dz in -far_reach..=far_reach {
            let c = center + IVec2::new(dx, dz);
            let d = dist_to_chunk(p, c);
            if d <= FAR_LOAD_DIST && !chunks.far.contains_key(&c) {
                wanted.push((d, c));
            }
        }
    }
    wanted.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, c) in wanted {
        if far_budget == 0 {
            break;
        }
        far_budget -= 1;
        let dims: Vec<Dims> = assets.houses.iter().map(|h| h.dims).collect();
        let leafy: Vec<bool> = (0..assets.trees.len()).map(|i| i >= 4).collect();
        let (body, glow) = far::build_far_chunk(c, &dims, &leafy);
        let tf = Transform::from_xyz(c.x as f32 * CHUNK, 0.0, c.y as f32 * CHUNK);
        let e1 = commands
            .spawn((
                Mesh3d(meshes.add(body)),
                MeshMaterial3d(assets.vertex_mat.clone()),
                tf,
                NotShadowCaster,
            ))
            .id();
        let e2 = commands
            .spawn((
                Mesh3d(meshes.add(glow)),
                MeshMaterial3d(assets.far_glow_mat.clone()),
                tf,
                NotShadowCaster,
            ))
            .id();
        chunks.far.insert(c, vec![e1, e2]);
    }

    let stale: Vec<IVec2> = chunks
        .far
        .keys()
        .copied()
        .filter(|c| dist_to_chunk(p, *c) > FAR_UNLOAD_DIST)
        .collect();
    for c in stale {
        if let Some(ents) = chunks.far.remove(&c) {
            for e in ents {
                commands.entity(e).despawn();
            }
        }
        chunks.far_hidden.remove(&c);
    }

    let keys: Vec<IVec2> = chunks.far.keys().copied().collect();
    for c in keys {
        let done = chunks.loaded.contains_key(&c) && !chunks.pending.iter().any(|(pc, _)| *pc == c);
        let hidden = chunks.far_hidden.contains(&c);
        if done != hidden {
            let vis = if done {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
            for e in &chunks.far[&c] {
                commands.entity(*e).insert(vis);
            }
            if done {
                chunks.far_hidden.insert(c);
            } else {
                chunks.far_hidden.remove(&c);
            }
        }
    }
}

fn update_lod(
    time: Res<Time>,
    mut acc: Local<f32>,
    player: Single<&Transform, With<Player>>,
    mut lods: Query<(&GlobalTransform, &mut Lod, &mut Mesh3d)>,
    mut hides: Query<(&GlobalTransform, &DistHide, &mut Visibility), Without<GroundTier>>,
    mut tiers: Query<(&GroundTier, &mut Visibility)>,
) {
    *acc += time.delta_secs();
    if *acc < 0.1 {
        return;
    }
    *acc = 0.0;
    let p = player.translation;
    let pxz = Vec2::new(p.x, p.z);
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
    for (t, mut vis) in &mut tiers {
        let d = pxz.clamp(t.min, t.max).distance(pxz);
        let shown = *vis != Visibility::Hidden;
        let slack = if shown { 3.0 } else { 0.0 };
        let want = d + slack >= t.from && d - slack < t.to;
        let target = if want {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != target {
            *vis = target;
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
                LOD_TREE,
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
                    DistHide { dist: 16.0 },
                ));
            }
        })
        .id()
}

fn spawn_lamp(commands: &mut Commands, a: &CityAssets, pos: Vec3, lit: bool) -> Entity {
    let glass_mat = if lit {
        a.lamp_glass_mat.clone()
    } else {
        a.window_dark.clone()
    };
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
                MeshMaterial3d(glass_mat),
                Transform::default(),
            ));
            if lit {
                p.spawn((
                    PointLight {
                        color: Color::srgb(1.0, 0.72, 0.38),
                        intensity: LAMP_LUMENS,
                        range: 12.0,
                        radius: 0.15,
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 4.4, 0.0),
                    Visibility::Hidden,
                    DistHide { dist: 30.0 },
                ));
            }
            p.spawn((
                Collider::cylinder(0.25, 4.6),
                Transform::from_xyz(0.0, 2.3, 0.0),
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
    let st = &a.streets[variant];
    let origin = Transform::from_xyz(ox, 0.0, oz);
    let whole = (Vec2::new(ox, oz), Vec2::new(ox + CHUNK, oz + CHUNK));
    ents.push(
        commands
            .spawn((
                Mesh3d(st.full.clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                origin,
                NotShadowCaster,
                Visibility::Hidden,
                GroundTier {
                    min: whole.0,
                    max: whole.1,
                    from: 0.0,
                    to: GROUND_FULL,
                },
            ))
            .id(),
    );
    ents.push(
        commands
            .spawn((
                Mesh3d(st.lite.clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                origin,
                NotShadowCaster,
                Visibility::Hidden,
                GroundTier {
                    min: whole.0,
                    max: whole.1,
                    from: GROUND_FULL,
                    to: f32::MAX,
                },
            ))
            .id(),
    );
    for (qi, lm) in st.litter.iter().enumerate() {
        let qmin = Vec2::new(
            ox + (qi % 2) as f32 * CHUNK * 0.5,
            oz + (qi / 2) as f32 * CHUNK * 0.5,
        );
        ents.push(
            commands
                .spawn((
                    Mesh3d(lm.clone()),
                    MeshMaterial3d(a.vertex_mat.clone()),
                    origin,
                    NotShadowCaster,
                    Visibility::Hidden,
                    GroundTier {
                        min: qmin,
                        max: qmin + Vec2::splat(CHUNK * 0.5),
                        from: 0.0,
                        to: LITTER_DIST,
                    },
                ))
                .id(),
        );
    }

    let near = ROAD_W + CURB_W + STRIP_W * 0.5;
    let far = CHUNK - CURB_W - STRIP_W * 0.5;

    for k in 0..2 {
        for edge in [near, far] {
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            let lit = rng.chance(LAMP_LIT_CHANCE);
            ents.push(spawn_lamp(
                commands,
                a,
                Vec3::new(ox + edge, 0.0, oz + t),
                lit,
            ));
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            let lit = rng.chance(LAMP_LIT_CHANCE);
            ents.push(spawn_lamp(
                commands,
                a,
                Vec3::new(ox + t, 0.0, oz + edge),
                lit,
            ));
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
    let win_mat = if rng.chance(WINDOW_LIT_CHANCE) {
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

            p.spawn((
                Collider::cuboid(d, h, w),
                Transform::from_xyz(hx_c, h * 0.5, 0.0),
            ));
            p.spawn((
                Collider::cuboid(2.65, 0.55, 4.8),
                Transform::from_xyz(x_front - 1.3, 0.275, 0.0),
            ));
            let run = 1.35;
            let rise: f32 = 0.55;
            let slope = rise.atan2(run);
            let ramp_len = (run * run + rise * rise).sqrt();
            let thick = 0.3;
            p.spawn((
                Collider::cuboid(ramp_len, thick, 2.6),
                Transform::from_xyz(
                    x_front - 2.6 - run * 0.5 + slope.sin() * thick * 0.5,
                    rise * 0.5 - slope.cos() * thick * 0.5,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_z(slope)),
            ));
            if let Some(zc) = v.dims.bay_z {
                p.spawn((
                    Collider::cuboid(1.0, houses::FLOOR_H, 2.5),
                    Transform::from_xyz(x_front - 0.5, houses::FLOOR_H * 0.5, zc),
                ));
            }

            p.spawn((
                Mesh3d(v.detail.clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                Transform::default(),
                NotShadowCaster,
                Visibility::Hidden,
                DistHide { dist: DETAIL_DIST },
            ));

            p.spawn((
                Mesh3d(a.unit_cube.clone()),
                MeshMaterial3d(a.path_mat.clone()),
                Transform::from_xyz(path_cx, 0.03, 0.0).with_scale(Vec3::new(path_len, 0.06, 1.8)),
                NotShadowCaster,
                DistHide { dist: PROP_CULL },
            ));

            p.spawn((
                Mesh3d(a.mailbox.clone()),
                MeshMaterial3d(a.vertex_mat.clone()),
                Transform::from_xyz(front_edge + 0.9, 0.0, 2.6),
                NotShadowCaster,
                DistHide { dist: PROP_CULL },
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
                    DistHide { dist: PROP_CULL },
                ));
            }

            for (pos, li, lyaw, ls) in &leaf_piles {
                p.spawn((
                    Mesh3d(a.leaf_piles[*li].clone()),
                    MeshMaterial3d(a.vertex_mat.clone()),
                    Transform::from_translation(*pos)
                        .with_rotation(Quat::from_rotation_y(*lyaw))
                        .with_scale(Vec3::splat(*ls)),
                    NotShadowCaster,
                    DistHide { dist: PROP_CULL },
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
    const SIZE: u32 = 1024;
    let moon_dir = MOON_DIR.normalize();
    let moon_r = MOON_RADIUS_DEG.to_radians();
    let moon_right = Vec3::Y.cross(moon_dir).normalize();
    let moon_up = moon_dir.cross(moon_right);
    let sun_dir = Vec3::new(0.45, 0.2, 0.87).normalize();
    let horizon = Vec3::from(HAZE_RGB);
    let zenith = Vec3::from(ZENITH_RGB);

    let mut crater_rng = GameRng::seeded(0x4D00_4E55);
    let craters: Vec<(Vec2, f32)> = (0..70)
        .map(|_| {
            let a = crater_rng.range(0.0, TAU);
            let r = crater_rng.f32().sqrt() * 0.92;
            (
                Vec2::new(a.cos() * r, a.sin() * r),
                crater_rng.range(0.025, 0.13),
            )
        })
        .collect();

    let mut data: Vec<u8> = Vec::with_capacity((SIZE * SIZE * 4 * 6) as usize);

    for face in 0..6u32 {
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
                let d = Vec3::new(dir.x, dir.y, -dir.z).normalize();

                let t = (d.y / 0.85).clamp(0.0, 1.0);
                let t = t.powf(0.6);
                let mut col = horizon.lerp(zenith, t);

                if d.y > 0.12 {
                    let h = hash2(px as i32, py as i32, face * 13 + 5);
                    if h > 0.9985 {
                        let k = (h - 0.9985) / 0.0015;
                        let fade = ((d.y - 0.12) / 0.3).clamp(0.0, 1.0);
                        col += Vec3::new(0.7, 0.75, 0.9) * (0.08 + 0.4 * k * k) * fade;
                    }
                }

                let cos_ang = d.dot(moon_dir).clamp(-1.0, 1.0);
                let ang = cos_ang.acos();
                if d.y > -0.05 {
                    let halo = (1.0 - ang / 0.5).clamp(0.0, 1.0);
                    col +=
                        Vec3::new(0.30, 0.38, 0.60) * (halo.powi(3) * 0.07 + halo.powi(8) * 0.12);
                }
                if ang < moon_r * 1.6 {
                    let mx = d.dot(moon_right) / moon_r.sin();
                    let my = d.dot(moon_up) / moon_r.sin();
                    let rr = (mx * mx + my * my).sqrt();
                    let edge = ((1.04 - rr) / 0.06).clamp(0.0, 1.0);
                    if edge > 0.0 {
                        let rc = rr.min(1.0);
                        let nz = (1.0 - rc * rc).sqrt();
                        let normal =
                            Vec3::new(mx.clamp(-1.0, 1.0), my.clamp(-1.0, 1.0), nz).normalize();
                        let mut albedo = 0.80;
                        let maria = fbm(mx * 2.3 + 11.0, my * 2.3 + 5.0, 77);
                        albedo -= ((maria - 0.46) * 3.0).clamp(0.0, 1.0) * 0.38;
                        albedo += (fbm(mx * 14.0, my * 14.0, 31) - 0.5) * 0.14;
                        let p = Vec2::new(mx, my);
                        let mut relief = 0.0;
                        for (cc, cr) in &craters {
                            let q = p.distance(*cc) / cr;
                            if q < 1.0 {
                                relief -= (1.0 - q * q) * 0.55;
                            } else if q < 1.25 {
                                relief += (1.0 - (q - 1.0) / 0.25) * 0.5;
                            }
                        }
                        albedo = (albedo + relief * 0.22).clamp(0.12, 1.0);
                        let diffuse = normal.dot(sun_dir).max(0.0).powf(0.8);
                        let lum = (0.10 + diffuse * 1.0) * albedo * (0.55 + 0.45 * nz.powf(0.3));
                        let moon_col = Vec3::new(0.93, 0.95, 1.0) * lum;
                        col = col.lerp(moon_col, edge);
                    }
                }

                let jitter = hash2(px as i32, py as i32, face * 31 + 7) - 0.5;
                let q = |c: f32| {
                    ((c.clamp(0.0, 1.0) * 255.0 + jitter)
                        .round()
                        .clamp(0.0, 255.0)) as u8
                };
                data.extend_from_slice(&[q(col.x), q(col.y), q(col.z), 255]);
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
                Mist,
                FogVolume {
                    fog_color: Color::srgb(0.6, 0.7, 1.0),
                    density_factor: MIST_DENSITY,
                    light_tint: Color::srgb(0.7, 0.8, 1.0),
                    light_intensity: 1.0,
                    ..default()
                },
                Transform::from_xyz(0.0, 4.0, 0.0).with_scale(MIST_SIZE),
            ));
            p.spawn((
                Head,
                Transform::from_xyz(0.0, 0.7, 0.0),
                Visibility::default(),
            ))
            .with_children(|h| {
                let mut cam = h.spawn((
                    PlayerCamera,
                    Camera3d::default(),
                    Camera {
                        hdr: true,
                        clear_color: ClearColorConfig::Custom(HAZE_COLOR),
                        ..default()
                    },
                    Tonemapping::TonyMcMapface,
                    Bloom {
                        intensity: 0.12,
                        ..Bloom::NATURAL
                    },
                    Smaa {
                        preset: SmaaPreset::High,
                    },
                    ColorGrading {
                        global: ColorGradingGlobal {
                            exposure: 0.4,
                            ..default()
                        },
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
                        color: HAZE_COLOR,
                        falloff: FogFalloff::Linear {
                            start: HAZE_START,
                            end: HAZE_END,
                        },
                        ..default()
                    },
                    VolumetricFog {
                        ambient_intensity: 0.0,
                        step_count: MIST_STEPS,
                        ..default()
                    },
                    Skybox {
                        image: skybox,
                        brightness: 1000.0,
                        ..default()
                    },
                ));
                if SSAO_ENABLED {
                    cam.insert(ScreenSpaceAmbientOcclusion {
                        quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Medium,
                        ..default()
                    });
                }
                cam.with_children(|c| {
                    let mut flash = c.spawn((
                        Flashlight,
                        SpotLight {
                            color: Color::srgb(1.0, 0.92, 0.75),
                            intensity: FLASHLIGHT_BASE,
                            range: 14.0,
                            radius: 0.04,
                            shadows_enabled: FLASHLIGHT_SHADOWS,
                            inner_angle: 0.3,
                            outer_angle: 0.7,
                            ..default()
                        },
                        Transform::from_xyz(0.2, -0.15, 0.0),
                    ));
                    if FLASHLIGHT_SHADOWS && FLASHLIGHT_BEAM {
                        flash.insert(VolumetricLight);
                    }
                });
            });
        });
}

fn make_vignette_image() -> Image {
    const SIZE: u32 = 256;
    let mut data = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for py in 0..SIZE {
        for px in 0..SIZE {
            let u = (px as f32 + 0.5) / SIZE as f32 * 2.0 - 1.0;
            let v = (py as f32 + 0.5) / SIZE as f32 * 2.0 - 1.0;
            let r = (u * u + v * v).sqrt();
            let t = ((r - 0.6) / 0.8).clamp(0.0, 1.0);
            let a = t * t * (3.0 - 2.0 * t);
            data.extend_from_slice(&[0, 0, 0, (a * 0.5 * 255.0) as u8]);
        }
    }
    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn setup_hud(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode {
            image: images.add(make_vignette_image()),
            ..default()
        },
        ZIndex(-1),
    ));
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
    commands.spawn((
        StatsText,
        Text::new("..."),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgba(0.9, 0.9, 0.5, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}

fn update_stats(
    time: Res<Time>,
    mut acc: Local<(f32, u32)>,
    entities: Query<Entity>,
    bodies: Query<&RigidBody>,
    chunks: Res<Chunks>,
    mut text: Single<&mut Text, With<StatsText>>,
) {
    acc.0 += time.delta_secs();
    acc.1 += 1;
    if acc.0 < 0.5 {
        return;
    }
    let fps = acc.1 as f32 / acc.0;
    let ms = acc.0 / acc.1 as f32 * 1000.0;
    let dynamic = bodies
        .iter()
        .filter(|b| matches!(b, RigidBody::Dynamic))
        .count();
    text.0 = format!(
        "{:.0} fps ({:.1} ms) | entities {} | dynamic {} | chunks {} | queue {}",
        fps,
        ms,
        entities.iter().count(),
        dynamic,
        chunks.loaded.len(),
        chunks.pending.len()
    );
    *acc = (0.0, 0);
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

fn fog_breathing(time: Res<Time>, mut mist: Single<&mut FogVolume, With<Mist>>) {
    mist.density_factor = MIST_DENSITY * (1.0 + 0.25 * (time.elapsed_secs() * 0.15).sin());
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
