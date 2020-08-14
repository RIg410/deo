use bevy::prelude::*;

#[derive(Debug)]
pub struct Plane {
}

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_plane.system());
    }
}

fn setup_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrComponents {
        mesh: asset_server.load("assets/objects/plane/plane.gltf").unwrap(),
        material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
        translation: Translation::new(0.0, 0.0, 0.0),
        ..Default::default()
    }).with(Plane{});
}