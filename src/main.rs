use bevy::{
    prelude::*,
    //utils::FloatOrd
};

const WIDTH: f32 = 500.;
const HEIGHT: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_boid)
        .run();
}

#[derive(Component)]
struct Boid;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.25, 0.75),
            custom_size: Some(Vec2::new(500.0, 500.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    commands.spawn(Camera2dBundle::default()); 
    for i in 0..5 {
        let j = i as f32;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(j*60.-100., 0., 0.))
                .with_rotation(Quat::from_rotation_z(j*30.0_f32.to_radians()))
                .with_scale(Vec3 { x: 0.5, y: 0.5, z: 1.0 }),
                ..default()
            },
            Boid,
        ));
        }
}

// fn steer_boid(
//     mut boids: Query<&mut Transform, With<Boid>>,
// ) {
//     for mut transform in &mut boids {
//         let closest_boid = boids
//             .iter()
//             .min_by_key(|boid_transform| {
//                 FloatOrd(Vec3::distance(boid_transform.translation,transform.translation))
//             })
//             .map(|closest| closest.translation);
//     }
// }

fn move_boid(
    mut boids: Query<&mut Transform, With<Boid>>,
) {
    for mut transform in &mut boids {
        let (_, angle) = transform.rotation.to_axis_angle();

        transform.translation.x += 5.0*(angle.cos());
        transform.translation.y += 5.*(angle.sin());

        if transform.translation.y < -HEIGHT/2. {transform.translation.y += 500.};
        if transform.translation.y >= HEIGHT/2. {transform.translation.y -= 500.};
        if transform.translation.x < -WIDTH/2. {transform.translation.x += 500.};
        if transform.translation.x >= WIDTH/2. {transform.translation.x -= 500.};

    }
}