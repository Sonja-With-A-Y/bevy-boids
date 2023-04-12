//Imports
use std::collections::HashMap;

use bevy::{
    prelude::*,
    math::f32::Vec3,
};

use bevy_embedded_assets::EmbeddedAssetPlugin;

//Configuration
const WIDTH: f32 = 100.;
const HEIGHT: f32 = 100.;
const POND_RADIIUS: f32 = 200.;
//const BACKGROUND_COLOR: bevy::prelude::Color = Color::rgb(0.263, 0.573, 0.945);

const NUMBER_OF_BOIDS: i32 = 50;
const BOID_SCALE: f32 = 2.;

const BOID_SPEED: f32 = 20.;
const BOID_ROTATE_SPEED: f32 = 0.5;

const SEPARATION_CIRCLE_RADIUS: f32 = 25.;
const SEPARATION_CONE_RADIUS: f32 = 20.;
const SEPARATION_CONE_ANGLE: f32 = 45.0;

const ALIGN_INCLUSION_RADIUS: f32 = 25.;

const WALL_AVOIDANCE_DISTANCE: f32 = 50.;
const WALL_AVOIDANCE_PUSH: f32 = 1500.;

//Main
fn main() {
    App::new()
//        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.build().add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),)
        .add_startup_system(setup)
        .add_systems((boid_force_calc, sympathy_force_calc, turn_boid, move_boid).chain())
        .run();
}

//Components
#[derive(Component)]
struct Boid;

#[derive(Component, Copy, Clone, Default, Resource)]
#[component(storage = "SparseSet")]
struct Force (Vec3);

//Systems
fn setup(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 150000.0,
            range: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0., 25.0),
        ..default()
    });

   //Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(150.0, 150.0, 80.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(POND_RADIIUS).into()),
        material: materials.add(Color::rgb(0.263, 0.573, 0.945).into()),
        ..default()
    });

  //  let duck_sprite: Handle<Image> = asset_server.load("duck.png");
    //Boids
    for i in 0..NUMBER_OF_BOIDS {
        let j = i as f32;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.408, 0.584, 0.506).into()),
//                texture: duck_sprite.clone().into(),
                transform: Transform::from_translation(Vec3::new(j*6., 0., 1.))
                .with_rotation(Quat::from_rotation_z((j*30.0_f32+1.).to_radians()))
                .with_scale(Vec3 { x: BOID_SCALE, y: BOID_SCALE, z: BOID_SCALE }),
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
        if transform1.translation.length() > POND_RADIIUS - WALL_AVOIDANCE_DISTANCE {
            forcesum += -WALL_AVOIDANCE_PUSH*transform1.translation;
        }
        
        //Boid relations setup
        for transform2 in &neighbour_query {
            if transform1 == transform2 {continue}

            let dif_vector = transform2.translation - transform1.translation;

            if dif_vector.length() < (transform1.translation - closest_boid).length() {
                closest_boid = transform2.translation;
            }

            //Boid avoidance close
            if dif_vector.length() < SEPARATION_CIRCLE_RADIUS {
                forcesum += -(dif_vector.normalize()*SEPARATION_CIRCLE_RADIUS - dif_vector);
            }

            //Boid alignment
            if dif_vector.length() < ALIGN_INCLUSION_RADIUS
                && transform2.local_x().angle_between(transform1.local_x()) < 90.0_f32.to_radians()
                {
                    forcesum += 15.*transform2.local_x();
                }

            if dif_vector.angle_between(transform1.local_x()) > SEPARATION_CONE_ANGLE.to_radians()/2. {
                continue
            }
            
            //Boid avoidance in sight
            if dif_vector.length() < SEPARATION_CONE_RADIUS {
                forcesum += -(dif_vector.normalize()*SEPARATION_CONE_RADIUS - dif_vector);
            }
        }

        //Boid loneliness
        forcesum += 2.*(closest_boid - transform1.translation);

        //First force application
        if forcesum.length() > 0.1 {
            commands.entity(entity).insert(Force(forcesum));
        }
    }

}

fn sympathy_force_calc(
    mut set: ParamSet<(
        Query<(Entity, &Transform, &Force), With<Boid>>,
        Query<(&Transform, &Force), With<Boid>>,
        Query<(Entity, &mut Force), With<Boid>>,
        )>,
    ) {
        
        let mut trans_map = HashMap::new();
        let mut force_map = HashMap::new();

        for (entity, transform1, _) in set.p0().iter() {
            trans_map.insert(
                entity.index(),
                (transform1.translation.x, transform1.translation.y),
                );
        }

        for (transform2, force2) in set.p1().iter() {
            for (entity, info) in trans_map.iter() {

                if (transform2.translation - info.0).length() < ALIGN_INCLUSION_RADIUS
                    && (transform2.translation - info.0).length() > SEPARATION_CIRCLE_RADIUS {
                    let force = force_map.entry(entity).or_insert(force2.0);
                    *force += force2.0*5.;
                }
            }
        }

        //Second force application
        for (entity, mut force) in set.p2().iter_mut() {
           if force_map.contains_key(&entity.index()) {
               force.0 += force_map[&entity.index()];
           }
        }

}

fn turn_boid(
    mut commands: Commands,
    mut boids: Query<(Entity, &mut Transform, &Force), With<Boid>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, force) in &mut boids {
        let real_turn_speed = BOID_ROTATE_SPEED * 360.0_f32.to_radians() * timer.delta_seconds();

        if force.0.angle_between(transform.local_x()) > 10.0_f32.to_radians() {
            if force.0.angle_between(transform.local_y()) < 90.0_f32.to_radians() {
                transform.rotate_z(real_turn_speed);            
            } else {
                transform.rotate_z(-real_turn_speed);

            }
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
    }
}
