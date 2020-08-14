use bevy::{
    prelude::*,
    property::{ron::deserialize_dynamic_properties, PropertyTypeRegistry},
    type_registry::TypeRegistry,
};
use serde::{Deserialize, Serialize};


pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub fn setup(mut commands: Commands) {
    // if let Ok(scene_handle) = asset_server.load::<DEO, _>("assets/scenes/cfg.scn") {
    //
    // } else {
    //
    // }
}

#[derive(Serialize, Deserialize, Clone, Properties, Default)]
pub struct DEO {
    window: Window,
    world: World,
}

#[derive(Serialize, Deserialize, Clone, Properties)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

impl Window {

}

impl Default for Window {
    fn default() -> Self {
        Window { width: 1280, height: 720, vsync: true }
    }
}

#[derive(Serialize, Deserialize, Clone, Properties, Default)]
pub struct World {
    plane: Plane,
}


#[derive(Serialize, Deserialize, Clone, Properties, Default)]
pub struct Plane {
    initial_position: Vec3,
    initial_direction: Vec3,
}
