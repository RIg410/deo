pub mod cam;
pub mod plane;
pub mod settings;
pub mod world;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(settings::SettingsPlugin)
        .add_plugin(world::WorldPlugin)
        .add_resource(Msaa { samples: 4 })
        .add_plugin(plane::PlanePlugin)
        .add_plugin(cam::CameraPlugin)
        .add_default_plugins()
        .run();
}
