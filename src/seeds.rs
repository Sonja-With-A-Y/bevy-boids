use rand::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Seed;

#[derive(Resource)]
pub struct SeedTimer {
    pub timer: Timer,
}

pub fn drop_seeds(
    mut commands: Commands,
    time: Res<Time>,
    mut seed_timer: ResMut<SeedTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    seed_timer.timer.tick(time.delta());

    let angle = (rand::thread_rng().gen_range(0..360) as f32).to_radians();
    let radius =
        rand::thread_rng().gen_range(0..(POND_RADIIUS - WALL_AVOIDANCE_DISTANCE) as i32) as f32;
    let seed_transform =
        Transform::from_translation(Vec3::new(angle.cos(), angle.sin(), 0.) * radius);

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

pub fn delete_seed(
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
