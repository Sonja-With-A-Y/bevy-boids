use std::f32::consts::PI;

use bevy::{
    prelude::*,
    //utils::FloatOrd,
    sprite::{MaterialMesh2dBundle},
    math::f32::Vec2
};

const WIDTH: f32 = 1000.;
const HEIGHT: f32 = 500.;

const BOID_SPEED: f32 = 5.;
const BOID_SCALE: f32 = 0.5;
const SEPARATION_CIRCLE_RADIUS: f32 = 150.*BOID_SCALE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(boid_force_calc)
        // .add_system(turn_boid)
        .add_system(move_boid)
        .run();
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Force {force: Vec2,}

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
    for i in 0..1 {
        //Test Circle
        commands.spawn((
            MaterialMesh2dBundle {
                mesh:meshes.add(shape::Circle::new(25.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..default()
            },
            //Force {force: Vec2::new(0., 0.)},
        ));

        //Separation Circle
        let circle = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(SEPARATION_CIRCLE_RADIUS/BOID_SCALE).into()).into(), 
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            },
        )).id();

        //Ahead Point
        let ahead = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
                ..default()
            },
            Ahead,
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
                transform: Transform::from_translation(Vec3::new(j*60.-100., 0., 1.))
                .with_rotation(Quat::from_rotation_z(j*30.0_f32.to_radians()))
                .with_scale(Vec3 { x: BOID_SCALE, y: BOID_SCALE, z: 1. }),
                ..default()
            },
            Boid,
        )).push_children(&[circle, ahead]);
    
    }
}

fn boid_force_calc(
    mut commands: Commands,
    boids: Query<(Entity, &GlobalTransform), With<Boid>>,
    collider_query: Query<&GlobalTransform, With<Boid>>,
) {
    for (entity, transform1) in &boids {
        let mut forcesum = Vec2::new(0., 0.);

        for transform2 in &collider_query {
            let transform1_2d = Vec2::new(transform1.translation().x, transform1.translation().y) ;
            let transform2_2d = Vec2::new(transform2.translation().x, transform2.translation().y) ;

            let dist = transform1_2d.distance(transform2_2d);
            
            if dist <= SEPARATION_CIRCLE_RADIUS
            {
                forcesum += (transform1_2d - transform2_2d).normalize_or_zero();
            }
            commands.entity(entity).insert(Force {force: forcesum});
            //println!("{:?}", forcesum)
        }
    }
}

// fn turn_boid(
//     mut boids: Query<(&mut Transform, &Children), With<Boid>>,
//     aheads: Query<&GlobalTransform>,
// ) {
//     for (mut transform, children) in &mut boids.iter() {

//         for &child in children.iter() {
//             let aheadloc = aheads.get(child).unwrap().translation();
            
//             println!("{:?}", aheadloc);
            
//             let target = aheadloc ;
//             // transform.rotate_z(1.0_f32.to_radians());
            
//             transform.rotation = transform.looking_at(target, Vec3::new(0., 0., 1.));
//         }
//     }
// }

fn move_boid(
    mut boids: Query<&mut Transform, With<Boid>>,
) {
    for mut transform in &mut boids {
        
        let angle = transform.rotation.z;

        transform.translation.x += BOID_SPEED*(angle.cos());
        transform.translation.y += BOID_SPEED*(angle.sin());

        if transform.translation.y < -HEIGHT/2. {transform.translation.y += HEIGHT};
        if transform.translation.y >= HEIGHT/2. {transform.translation.y -= HEIGHT};
        if transform.translation.x < -WIDTH/2. {transform.translation.x += WIDTH};
        if transform.translation.x >= WIDTH/2. {transform.translation.x -= WIDTH};

    }
}