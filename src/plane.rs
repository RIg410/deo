use crate::settings::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Plane;

#[derive(Debug)]
pub struct Vector;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_plane.system())
            .add_system(plane_control_system.system())
            .add_system(plane_movement_system.system());
    }
}

fn setup_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cfg: Res<DEO>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_cfg = &cfg.world.plane;

    commands
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/objects/plane/plane.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::from(plane_cfg.initial_position),
            rotation: Rotation::from(plane_cfg.initial_direction),
            ..Default::default()
        })
        .with(Plane);
}

fn plane_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut plane_query: Query<(&Plane, &mut Rotation)>,
) {
    for (_, mut rotation) in &mut plane_query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            rotation.0 *= Quat::from_rotation_x(-0.01);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation.0 *= Quat::from_rotation_x(0.01);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            rotation.0 *= Quat::from_rotation_z(-0.01);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            rotation.0 *= Quat::from_rotation_z(0.01);
        }
        if keyboard_input.pressed(KeyCode::A) {
            rotation.0 *= Quat::from_rotation_y(-0.01);
        }
        if keyboard_input.pressed(KeyCode::D) {
            rotation.0 *= Quat::from_rotation_y(0.01);
        }
    }
}

fn plane_movement_system(
    time: Res<Time>,
    mut plane_query: Query<(&Plane, &mut Translation, &Rotation)>,
) {
    for (_, mut translation, rotation) in &mut plane_query.iter() {
        let rt = Mat4::from_rotation_translation(rotation.0, translation.0);
        let rt = rt * Mat4::from_translation(Vec3::new(0.01, 0.0, 0.0));
        let (_, _, tr) = rt.to_scale_rotation_translation();
        translation.0 = tr;
    }
}
