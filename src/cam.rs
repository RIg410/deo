use crate::plane::Plane;
use crate::settings::DEO;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera_setup.system())
            .add_system(movement_system.system());
    }
}

pub struct Camera;

fn camera_setup(mut commands: Commands, cfg: Res<DEO>) {
    let plane_cfg = &cfg.world.plane;
    let camera_cfg = &cfg.world.camera;
    let target = plane_cfg.initial_position + camera_cfg.relative_target;
    let eye = plane_cfg.initial_position + camera_cfg.relative_position;

    commands
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                eye,
                target,
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Camera);
}

fn movement_system(
    cfg: Res<DEO>,
    mut plane_query: Query<(&Plane, &Translation, &Rotation)>,
    mut cam_query: Query<(&Camera, &mut Transform)>,
) {
    for (_, plane_translation, plane_rotation) in &mut plane_query.iter() {
        let camera_cfg = &cfg.world.camera;
        let plane_matrix = Mat4::from_rotation_translation(plane_rotation.0, plane_translation.0);

        let plane_rotation_matrix =
            Mat4::from_rotation_translation(plane_rotation.0, Vec3::new(0.0, 0.0, 0.0));
        let camera_matrix = plane_matrix * Mat4::from_translation(camera_cfg.relative_position);
        for (_, mut transform) in &mut cam_query.iter() {
            *transform = Transform::new_sync_disabled(Mat4::face_toward(
                camera_matrix.to_scale_rotation_translation().2,
                plane_translation.0,
                plane_rotation_matrix.transform_point3(Vec3::new(0.0, 1.0, 0.0)),
            ));
        }
    }
}
