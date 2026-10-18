use bevy::{
    asset::RenderAssetUsages, light::NotShadowCaster, mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology,
};

use crate::geom::{Mat, MeshData};
use crate::layout::House;
use crate::math::V3;

#[derive(Resource)]
pub struct HouseRes(pub House);

pub fn to_vec3(v: V3) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

fn mesh_from(md: &MeshData) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, md.pos.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, md.nor.clone())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, md.uv.clone())
    .with_inserted_indices(Indices::U32(md.idx.clone()))
}

fn texture(assets: &AssetServer, name: &str) -> Handle<Image> {
    assets.load(format!("textures/{name}.ktx2"))
}

fn material_for(m: Mat, assets: &AssetServer) -> StandardMaterial {
    let textured = |name: &str, tint: Color, rough: f32, refl: f32| StandardMaterial {
        base_color: tint,
        base_color_texture: Some(texture(assets, name)),
        perceptual_roughness: rough,
        reflectance: refl,
        ..default()
    };
    let plain = |c: Color, rough: f32| StandardMaterial {
        base_color: c,
        perceptual_roughness: rough,
        reflectance: 0.2,
        ..default()
    };
    match m {
        Mat::Floor => textured("wood_floor", Color::srgb(0.82, 0.76, 0.7), 0.62, 0.3),
        Mat::Paper => textured("wallpaper", Color::srgb(0.86, 0.82, 0.74), 0.95, 0.12),
        Mat::Plaster => textured("plaster", Color::srgb(0.84, 0.82, 0.78), 0.95, 0.12),
        Mat::Tile => textured("tile", Color::srgb(0.9, 0.95, 0.92), 0.3, 0.5),
        Mat::Stucco => textured("stucco", Color::srgb(0.8, 0.84, 0.82), 0.95, 0.1),
        Mat::Concrete => textured("concrete", Color::srgb(0.85, 0.85, 0.83), 0.9, 0.1),
        Mat::Trim => plain(Color::srgb(0.2, 0.15, 0.11), 0.7),
        Mat::Wood => textured("wood_floor", Color::srgb(0.55, 0.5, 0.45), 0.6, 0.3),
        Mat::Roof => textured("concrete", Color::srgb(0.22, 0.25, 0.3), 0.85, 0.1),
        Mat::Door => textured("wood_floor", Color::srgb(0.42, 0.34, 0.28), 0.6, 0.3),
        Mat::Frame => plain(Color::srgb(0.56, 0.54, 0.48), 0.8),
        Mat::Glass => StandardMaterial {
            base_color: Color::srgba(0.55, 0.66, 0.72, 0.1),
            perceptual_roughness: 0.06,
            reflectance: 0.9,
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            cull_mode: None,
            ..default()
        },
    }
}

pub fn spawn_house(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    house: Res<HouseRes>,
) {
    for m in Mat::ALL {
        let md = &house.0.meshes[m.index()];
        if md.idx.is_empty() {
            continue;
        }
        let entity = commands
            .spawn((
                Mesh3d(meshes.add(mesh_from(md))),
                MeshMaterial3d(materials.add(material_for(m, &assets))),
                Transform::default(),
            ))
            .id();
        if m == Mat::Glass {
            commands.entity(entity).insert(NotShadowCaster);
        }
    }

    let size = 80.0;
    let tile = 0.22;
    let y = crate::layout::GROUND;
    let ground = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-size + 8.0, y, -size + 6.0],
            [size + 8.0, y, -size + 6.0],
            [size + 8.0, y, size + 6.0],
            [-size + 8.0, y, size + 6.0],
        ],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [(-size + 8.0) * tile, (-size + 6.0) * tile],
            [(size + 8.0) * tile, (-size + 6.0) * tile],
            [(size + 8.0) * tile, (size + 6.0) * tile],
            [(-size + 8.0) * tile, (size + 6.0) * tile],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 3, 2, 0, 2, 1]));
    commands.spawn((
        Mesh3d(meshes.add(ground)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.68, 0.6),
            base_color_texture: Some(texture(&assets, "ground")),
            perceptual_roughness: 0.97,
            reflectance: 0.05,
            ..default()
        })),
        Transform::default(),
    ));
}
