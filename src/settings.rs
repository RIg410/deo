use bevy::prelude::*;
use bevy_ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::fs;

const CFG: &'static str = "assets/settings/cfg.ron";

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let content = match fs::read_to_string(CFG) {
            Ok(content) => Some(content),
            Err(err) => {
                println!("Failed to load settings: {}", err);
                None
            }
        };

        let deo = if let Some(content) = content {
            bevy_ron::de::from_str::<DEO>(&content).unwrap()
        } else {
            let deo = DEO::default();
            fs::write(
                CFG,
                bevy_ron::ser::to_string_pretty(&deo, PrettyConfig::default()).unwrap(),
            )
            .unwrap();
            deo
        };

        app.add_resource(WindowDescriptor {
            width: deo.window.width,
            height: deo.window.height,
            title: "deo".to_string(),
            vsync: deo.window.vsync,
        })
        .add_resource(deo);
    }
}

#[derive(Serialize, Deserialize, Clone, Properties, Default)]
pub struct DEO {
    pub window: Window,
    pub world: World,
}

#[derive(Serialize, Deserialize, Clone, Properties)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            width: 1280,
            height: 720,
            vsync: true,
        }
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
