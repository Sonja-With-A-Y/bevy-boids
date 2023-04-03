use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    math::f32::{ Vec2, Vec3 },
    ecs::system::Command,
};

const WIDTH: f32 = 1500.;
const HEIGHT: f32 = 750.;

const BOID_SPEED: f32 = 150.;
const BOID_ROTATE_SPEED: f32 = 0.5;
const BOID_SCALE: f32 = 0.25;
const SEPARATION_CIRCLE_RADIUS: f32 = 75.;
const SEPARATION_CONE_RADIUS: f32 = 200.;
const SEPARATION_CONE_ANGLE: f32 = 45.0;
const ALIGN_INCLUSION_RADIUS: f32 = 150.;

//Dracula colours
const DRAC_BACKGROUND: Color = Color::rgb(40./255., 42./255., 54./255.);
const DRAC_PURPLE: Color = Color::rgb(189./255., 147./255., 249./255.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems((boid_force_calc, turn_boid, /*sympathy_force_calc,*/ move_boid).chain())
        .run();
}

#[derive(Component)]
struct Boid;

#[derive(Component, Copy, Clone, Default, Resource)]
#[component(storage = "SparseSet")]
struct Force (Vec3);

struct AddForce(Vec3);


fn setup(
    mut commands: Commands,
) {
    //Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: DRAC_BACKGROUND,
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
    //Camera
    commands.spawn(Camera2dBundle::default()); 

    //Boids
    for i in 0..50 {
        
        //Rectangle
        let j = i as f32;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: DRAC_PURPLE,
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new((j*60.)-100.+1., 0., 1.))
                .with_rotation(Quat::from_rotation_z((j*30.0_f32+1.).to_radians()))
                .with_scale(Vec3 { x: BOID_SCALE, y: BOID_SCALE, z: 1. }),
                ..default()
            },
            Boid,
        ));
    
    }
}

fn boid_force_calc(
    mut commands: Commands,
    boids: Query<(Entity, &Transform), With<Boid>>,
    neighbour_query: Query<&Transform, With<Boid>>,
) {
    for (entity, transform1) in &boids {

        let mut forcesum = Vec3::new(0., 0., 0.);
        let mut closest_boid = Vec3::new(10000., 10000., 10000.);

        //Wall avoidance
        if transform1.translation.x < -WIDTH/2. + 150. {
            forcesum += 175.*Vec3::new(1., 0., 0.);
        } else if transform1.translation.x > WIDTH/2. - 150. {
            forcesum += 175.*Vec3::new(-1., 0., 0.);
        }

        if transform1.translation.y < -HEIGHT/2. + 150. {
            forcesum += 175.*Vec3::new(0., 1., 0.);
        } else if transform1.translation.y > HEIGHT/2. - 150. {
            forcesum += 175.*Vec3::new(0., -1., 0.);
        }
        
        //Boid avoidance
        for transform2 in &neighbour_query {
            if transform1 == transform2 {continue}

            let dif_vector = transform2.translation - transform1.translation;

            if dif_vector.length() < (transform1.translation - closest_boid).length() {
                closest_boid = transform2.translation;
            }

            if dif_vector.length() < SEPARATION_CIRCLE_RADIUS {
                forcesum += -(dif_vector.normalize()*SEPARATION_CIRCLE_RADIUS - dif_vector);
            }

            if dif_vector.angle_between(transform1.local_x()) > SEPARATION_CONE_ANGLE.to_radians()/2. {
                continue
            }
            
            if dif_vector.length() < SEPARATION_CONE_RADIUS {
                forcesum += -(dif_vector.normalize()*SEPARATION_CONE_RADIUS - dif_vector);
            }
        }
        
        //Boid loneliness
        forcesum += 1.*(closest_boid - transform1.translation);

        //First force application
        if forcesum.length() > 0.1 {
            commands.entity(entity).insert(Force(forcesum));
        }
    }

}

fn sympathy_force_calc(
        mut commands: Commands,
        mut boids: Query<(Entity, &Force, &Transform), With<Boid>>,
        neighbour_query: Query<(&Transform, &Force), With<Boid>>,
    ) {
    //Sympathy
    for (entity, &force1, transform1) in boids.iter_mut() {
        let mut forcesum = Vec3::new(0., 0., 0.);

        for (transform2, force2) in neighbour_query.iter() {

            if (transform2.translation - transform1.translation).length() < ALIGN_INCLUSION_RADIUS {
                forcesum += 5.* force2.0;
            }
        }

        //Second force application
        if forcesum.length() > 0.1 {
            commands.entity(entity).remove::<Force>();
        }
    }
}

fn turn_boid(
    mut commands: Commands,
    mut boids: Query<(Entity, &mut Transform, &Force), With<Boid>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, force) in &mut boids {
        let real_turn_speed = force.0.length() * 0.01 * BOID_ROTATE_SPEED * TAU * timer.delta_seconds();

        if force.0.angle_between(transform.local_y()) < 90.0_f32.to_radians() {
            transform.rotate_z(real_turn_speed);            
        } else {
            transform.rotate_z(-real_turn_speed);

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
