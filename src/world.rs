use bevy::prelude::*;

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
    commands
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/objects/terrain/terrain.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.1, 0.2, 0.4).into()),
            translation: Translation::default(),
            rotation: Default::default(),
            scale: Scale::from(1000.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 10000.0, 4.0),
            ..Default::default()
        });
}
