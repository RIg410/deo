use bevy::prelude::*;
use bevy::render::mesh::{VertexAttribute, VertexAttributeValues};
use std::borrow::Borrow;

pub struct Terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = asset_server
        .load_sync(&mut meshes, "assets/objects/terrain/terrain.gltf")
        .unwrap();

    let scale = Scale::from(1000.0);
    let terrain = meshes.get(&mesh).unwrap();
    let collision_index = CollisionIndex::new(
        terrain,
        Mat4::from_scale(Vec3::new(scale.0, scale.0, scale.0)),
    );

    commands
        .spawn(PbrComponents {
            mesh,
            material: materials.add(Color::rgb(0.2, 0.7, 0.3).into()),
            translation: Translation::default(),
            rotation: Default::default(),
            scale,
            ..Default::default()
        })
        .with(Terrain)
        .with(collision_index)
        .spawn(LightComponents {
            translation: Translation::new(4.0, 10000.0, 4.0),
            ..Default::default()
        });
}

#[derive(Default, Debug)]
pub struct CollisionIndex {
   pub vertex: Vec<Vec3>,
}

impl CollisionIndex {
    pub fn new(mesh: &Mesh, matrix: Mat4) -> CollisionIndex {
        let vertex = mesh
            .attributes
            .iter()
            .find(|attributes| attributes.name.as_ref() == VertexAttribute::POSITION)
            .map(|att| match &att.values {
                VertexAttributeValues::Float3(val) => val
                    .iter()
                    .map(|val| Vec3::new(val[0], val[1], val[2]))
                    .map(|point| matrix.transform_point3(point))
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .unwrap_or_else(|| vec![]);

        CollisionIndex { vertex }
    }


}
