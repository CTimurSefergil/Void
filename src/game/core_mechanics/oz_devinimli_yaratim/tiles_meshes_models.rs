use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources);
}

#[derive(Resource)]
pub struct TileMeshes {
    pub ground: Handle<Mesh>,
    pub wall: Handle<Mesh>,
    pub tree: Handle<Mesh>,
    pub column: Handle<Mesh>,
    pub placeholder: Handle<Mesh>,
}

#[derive(Resource)]
pub struct TileMaterials {
    pub ground: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub tree: Handle<StandardMaterial>,
    pub column: Handle<StandardMaterial>,
    pub placeholder: Handle<StandardMaterial>,
}

fn setup_tile_resources(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let tile_meshes = TileMeshes {
        ground: mesh_assets.add(Cylinder::new(0.8, 3.0)),
        wall: mesh_assets.add(Cylinder::new(0.8, 3.0)),
        tree: mesh_assets.add(Cylinder::new(0.8, 3.0)),
        column: mesh_assets.add(Cylinder::new(0.8, 3.0)),
        placeholder: mesh_assets.add(Circle::new(1.0)),
    };

    let tile_materials = TileMaterials {
        ground: material_assets.add(StandardMaterial {
            base_color: Color::BLACK,
            ..Default::default()
        }),
        tree: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.7, 0.2),
            ..Default::default()
        }),
        wall: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.2, 0.1),
            ..Default::default()
        }),
        column: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.6, 0.6),
            ..Default::default()
        }),
        placeholder: material_assets.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..Default::default()
        }),
    };

    commands.insert_resource(tile_meshes);
    commands.insert_resource(tile_materials);
}
