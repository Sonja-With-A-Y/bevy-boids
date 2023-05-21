
use crate::*;

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.x = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();


    let speed = 30.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::Up) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    
    if keyboard.pressed(KeyCode::Down) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Left) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Right) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Z, rotate_speed * time.delta_seconds())
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Z, -rotate_speed * time.delta_seconds())
    }

}
