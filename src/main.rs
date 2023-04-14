//Imports
use std::collections::HashMap;
use std::time::Duration;
use rand::prelude::*;

use bevy::{
    pbr::prelude::*,
    prelude::*,
    math::f32::Vec3,
};

use bevy_embedded_assets::EmbeddedAssetPlugin;

//Configuration
const POND_RADIIUS: f32 = 200.;

const NUMBER_OF_BOIDS: i32 = 50;
const BOID_SCALE: f32 = 20.;

const BOID_SPEED: f32 = 20.;
const BOID_ROTATE_SPEED: f32 = 0.5;

const SEPARATION_CIRCLE_RADIUS: f32 = 25.;
const BOID_SIGHT_RANGE: f32 = 20.;
const BOID_SIGHT_ANGLE: f32 = 45.0;

const ALIGN_INCLUSION_RADIUS: f32 = 25.;

const WALL_AVOIDANCE_DISTANCE: f32 = 50.;
const WALL_AVOIDANCE_PUSH: f32 = 15.;

const SEED_EAT_RANGE: f32 = 5.;
const SEED_SPAWN_RATE: u64 = 5;
const HUNGER_RANGE: f32 = 80.;
const HUNGER_FACTOR: f32 = 80.;

#[derive(Resource)]
struct GameAssets {
    boid_scene: Handle<Scene>,
}

//Main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build()
        .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),)
        .add_startup_system(asset_loading.in_base_set(StartupSet::PreStartup))
        .add_startup_system(setup)
        .add_systems((
                boid_force_calc,
                sympathy_force_calc,
                turn_boid,
                move_boid,
                drop_seeds,
                delete_seed,
        ).chain())
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        boid_scene: assets.load("duck.glb#Scene0"),
    });
}

//Components
#[derive(Component)]
struct Boid;

#[derive(Resource)]
struct SeedTimer { timer: Timer }

#[derive(Component)]
struct Seed;

#[derive(Component, Copy, Clone, Default, Resource)]
#[component(storage = "SparseSet")]
struct Force (Vec3);

fn setup(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    //Light
    commands.insert_resource(AmbientLight {
        brightness: 1.,
        ..default()
    });

   //Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(150.0, 150.0, 80.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    //Pond
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(POND_RADIIUS).into()),
        material: materials.add(Color::rgb(0.263, 0.573, 0.945).into()),
        ..default()
    });

    //Boids
    for i in 0..NUMBER_OF_BOIDS {
        let j = i as f32;
        commands.spawn((
            SceneBundle {
                scene: game_assets.boid_scene.clone(),
                transform: Transform::from_translation(Vec3::new(j*6., 0., 1.))
                .with_rotation(Quat::from_rotation_z((j*30.0_f32+1.).to_radians()))
                .with_scale(Vec3 { x: BOID_SCALE, y: BOID_SCALE, z: BOID_SCALE }),
                ..Default::default()
            },
            Boid,
        ));
    }

    //Start see timer
    commands.insert_resource(SeedTimer {
        timer:Timer::new(Duration::from_secs(SEED_SPAWN_RATE), TimerMode::Repeating),
    })
}

fn boid_force_calc(
    mut commands: Commands,
    boids: Query<(Entity, &Transform), With<Boid>>,
    neighbour_query: Query<&Transform, With<Boid>>,
    seeds: Query<&Transform, With<Seed>>,
) {
    for (entity, transform1) in &boids {
        let mut forcesum = Vec3::new(0., 0., 0.);
        let mut closest_boid = Vec3::new(10000., 10000., 10000.);

        //Pond edge avoidance
        if transform1.translation.length() > POND_RADIIUS - WALL_AVOIDANCE_DISTANCE {
            forcesum += -WALL_AVOIDANCE_PUSH*transform1.translation;
        }

        //Seed sighted
        for seed in &seeds {
            if transform1.translation.distance(seed.translation) < HUNGER_RANGE {
                forcesum += (seed.translation - transform1.translation).normalize()*HUNGER_FACTOR;
            }
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

            //Boid avoidance in sight
            if dif_vector.angle_between(transform1.local_x()) > BOID_SIGHT_ANGLE.to_radians()/2. {
                continue
            }

            if dif_vector.length() < BOID_SIGHT_RANGE {
                forcesum += -(dif_vector.normalize()*BOID_SIGHT_RANGE - dif_vector);
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

        //Maps boid index to translation
        for (entity, transform1, _) in set.p0().iter() {
            trans_map.insert(
                entity.index(),
                (transform1.translation.x, transform1.translation.y),
                );
        }

        //Maps boid index to force
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

fn drop_seeds(
    mut commands: Commands,
    time: Res<Time>,
    mut seed_timer: ResMut<SeedTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    seed_timer.timer.tick(time.delta());

    let angle = (rand::thread_rng().gen_range(0..360) as f32).to_radians();
    let radius = rand::thread_rng().gen_range(0..(POND_RADIIUS-WALL_AVOIDANCE_DISTANCE) as i32) as f32;
    let seed_transform = Transform::from_translation(Vec3::new(
        angle.cos(),
        angle.sin(),
        0.
    )*radius);

    if seed_timer.timer.finished() {
        commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Capsule::default().into()),
                    material: materials.add(Color::rgb(0.8, 0.6, 0.0).into()),
                    transform: seed_transform,
                    ..default()
                },
                Seed,
        ));
    }
}

fn delete_seed(
    mut commands: Commands,
    seeds: Query<(Entity, &Transform), With<Seed>>,
    boids: Query<&Transform, With<Boid>>,
) {
    for (seed, seed_transform) in &seeds {
        for boid in &boids {
            if seed_transform.translation.distance(boid.translation) < SEED_EAT_RANGE {
                commands.entity(seed).despawn_recursive();
            }
        }
    }
}
