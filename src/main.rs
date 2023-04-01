use std::f32::consts::{TAU};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle},
    math::f32::Vec2
};

const WIDTH: f32 = 1000.;
const HEIGHT: f32 = 500.;

const BOID_SPEED: f32 = 150.;
const BOID_ROTATE_SPEED: f32 = 0.5;
const BOID_SCALE: f32 = 0.5;
const SEPARATION_CIRCLE_RADIUS: f32 = 200.;
const SEPARATION_CONE_ANGLE: f32 = 45.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems((boid_force_calc, turn_boid, move_boid).chain())
        .run();
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Force {force: Vec3,}

#[derive(Component)]
struct Ahead;

#[derive(Component)]
struct CollisionEvent;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.25, 0.75),
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
    //Camera
    commands.spawn(Camera2dBundle::default()); 

    //Boids
    for i in 0..20 {
        
        //Separation Circle
        let circle = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(SEPARATION_CIRCLE_RADIUS/BOID_SCALE).into()).into(), 
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                visibility: Visibility::Hidden,
                ..default()
            },
        )).id();

        //Rectangle
        let j = i as f32;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new((j*60.)-100.+1., 0., 1.))
                .with_rotation(Quat::from_rotation_z((j*30.0_f32+1.).to_radians()))
                .with_scale(Vec3 { x: BOID_SCALE, y: BOID_SCALE, z: 1. }),
                ..default()
            },
            Boid,
        )).push_children(&[circle]);
    
    }
}

fn boid_force_calc(
    mut commands: Commands,
    boids: Query<(Entity, &Transform), With<Boid>>,
    collider_query: Query<&Transform, With<Boid>>,
) {
    for (entity, transform1) in &boids {

        let mut forcesum = Vec3::new(0., 0., 0.);

        for transform2 in &collider_query {
            if transform1 == transform2 {continue}

            let dif_vector = transform2.translation - transform1.translation;

            if dif_vector.angle_between(transform1.local_x()) > SEPARATION_CONE_ANGLE.to_radians()/2. {
                continue
            }
            
            if dif_vector.length() < SEPARATION_CIRCLE_RADIUS {
                forcesum += -dif_vector;
                // -(transform1.translation() - transform2.translation());
            }
            if forcesum.length() > 0. {
                commands.entity(entity).insert(Force {force: forcesum});
            }
        }
    }
}

fn turn_boid(
    mut commands: Commands,
    mut boids: Query<(Entity, &mut Transform, &Force), With<Boid>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, force) in &mut boids {
        
        println!("{:?}", force.force);

        if force.force.angle_between(transform.local_y()) < 90.0_f32.to_radians() {
            transform.rotate_z(BOID_ROTATE_SPEED * TAU * timer.delta_seconds());            
        } else {
            transform.rotate_z(-BOID_ROTATE_SPEED * TAU * timer.delta_seconds());            

        }
        commands.entity(entity).remove::<Force>();

    }
}

fn move_boid(
    mut boids: Query<&mut Transform, With<Boid>>,
    timer: Res<Time>,
) {
    for mut transform in &mut boids {

        let direction = transform.local_x();
        transform.translation += direction * BOID_SPEED * timer.delta_seconds();

        if transform.translation.y < -HEIGHT/2. {transform.translation.y += HEIGHT};
        if transform.translation.y >= HEIGHT/2. {transform.translation.y -= HEIGHT};
        if transform.translation.x < -WIDTH/2. {transform.translation.x += WIDTH};
        if transform.translation.x >= WIDTH/2. {transform.translation.x -= WIDTH};

    }
}