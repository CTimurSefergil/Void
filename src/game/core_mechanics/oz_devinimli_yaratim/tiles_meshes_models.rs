use bevy::prelude::*;

pub const GROUND: [f32; 3] = [5.0, 0.1, 5.0];
pub const WALL: [f32; 3] = [4.8, 5.0, 4.8];
pub const CORNER: [f32; 3] = [4.8, 5.0, 4.8];
pub const CHEST: [f32; 3] = [1.5, 0.8, 1.0];
pub const PLACEHOLDER: [f32; 3] = [1.0, 1.0, 1.0];

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources);
}

#[derive(Resource)]
pub struct TileMeshes {
    pub ground: Handle<Mesh>,
    pub wall: Handle<Mesh>,
    pub corner: Handle<Mesh>,
    pub chest: Handle<Mesh>,
    pub placeholder: Handle<Mesh>,
}

#[derive(Resource)]
pub struct TileMaterials {
    pub ground: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub corner: Handle<StandardMaterial>,
    pub chest: Handle<StandardMaterial>,
    pub placeholder: Handle<StandardMaterial>,
}

fn setup_tile_resources(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = mesh_assets.add(Mesh::from(Cuboid::new(GROUND[0], GROUND[1], GROUND[2])));
    let wall_mesh = mesh_assets.add(Mesh::from(Cuboid::new(WALL[0], WALL[1], WALL[2])));
    let corner_mesh = mesh_assets.add(Mesh::from(Cuboid::new(CORNER[0], CORNER[1], CORNER[2])));
    let chest_mesh = mesh_assets.add(Mesh::from(Cuboid::new(CHEST[0], CHEST[1], CHEST[2])));
    let placeholder_mesh = mesh_assets.add(Mesh::from(Cuboid::new(
        PLACEHOLDER[0],
        PLACEHOLDER[1],
        PLACEHOLDER[2],
    )));

    let tile_meshes = TileMeshes {
        ground: ground_mesh,
        wall: wall_mesh,
        corner: corner_mesh,
        chest: chest_mesh,
        placeholder: placeholder_mesh,
    };

    let tile_materials = TileMaterials {
        ground: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.5, 0.0),
            ..Default::default()
        }),
        wall: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        }),
        corner: material_assets.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.3, 0.1),
            ..Default::default()
        }),
        chest: material_assets.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.843, 0.0),
            metallic: 0.8,
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
