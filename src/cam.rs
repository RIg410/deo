use bevy::prelude::*;
use crate::plane;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera_setup.system());
    }
}

fn camera_setup(
    mut commands: Commands,
) {
        commands.spawn(Camera3dComponents {
        transform: Transform::new_sync_disabled(Mat4::face_toward(
            Vec3::new(-10.0, 5.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    });
}