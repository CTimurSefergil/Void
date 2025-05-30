use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources);
}

#[derive(Resource)]
pub struct TileMeshes {
    pub sphere: Handle<Mesh>,
    pub cube: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
}

#[derive(Resource)]
pub struct TileMaterials {
    pub white: Handle<StandardMaterial>,
    pub green: Handle<StandardMaterial>,
    pub brown: Handle<StandardMaterial>,
    pub gray: Handle<StandardMaterial>,
}

fn setup_tile_resources(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let tile_meshes = TileMeshes {
        sphere: mesh_assets.add(Sphere::new(1.0)),
        cube: mesh_assets.add(Cuboid::new(4.0, 1.0, 4.0)),
        cylinder: mesh_assets.add(Cylinder::new(0.5, 3.0)),
    };

    let tile_materials = TileMaterials {
        white: material_assets.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        green: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.7, 0.2),
            ..Default::default()
        }),
        brown: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.2, 0.1),
            ..Default::default()
        }),
        gray: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.6, 0.6),
            ..Default::default()
        }),
    };

    commands.insert_resource(tile_meshes);
    commands.insert_resource(tile_materials);
}
