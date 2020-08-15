pub mod cam;
pub mod plane;
pub mod settings;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(settings::SettingsPlugin)
        .add_resource(Msaa { samples: 4 })
        .add_plugin(plane::PlanePlugin)
        .add_plugin(cam::CameraPlugin)
        .add_startup_system(setup.system())
        .add_default_plugins()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        });
}
