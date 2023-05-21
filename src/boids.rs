use crate::*;

#[derive(Component)]
pub struct Boid;

#[derive(Component, Copy, Clone, Default, Resource)]
#[component(storage = "SparseSet")]
pub struct Force(Vec3);

pub fn boid_force_calc(
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
            forcesum += -WALL_AVOIDANCE_PUSH * transform1.translation;
        }

        //Seed sighted
        for seed in &seeds {
            if transform1.translation.distance(seed.translation) < HUNGER_RANGE {
                forcesum += (seed.translation - transform1.translation)
                .normalize() * HUNGER_FACTOR;
            }
        }

        //Boid relations setup
        for transform2 in &neighbour_query {
            if transform1 == transform2 {
                continue;
            }
            let dif_vector = transform2.translation - transform1.translation;

            if dif_vector.length() < (transform1.translation - closest_boid).length() {
                closest_boid = transform2.translation;
            }

            //Boid avoidance close
            if dif_vector.length() < SEPARATION_CIRCLE_RADIUS {
                forcesum += -(dif_vector.normalize() * SEPARATION_CIRCLE_RADIUS - dif_vector);
            }

            //Boid alignment
            if dif_vector.length() < ALIGN_INCLUSION_RADIUS
                && transform2.local_x().angle_between(transform1.local_x()) < 90.0_f32
                .to_radians()
            {
                forcesum += 15. * transform2.local_x();
            }

            //Boid avoidance in sight
            if dif_vector.angle_between(transform1.local_x())
                > BOID_SIGHT_ANGLE.to_radians() / 2. {
                continue;
            }

            if dif_vector.length() < BOID_SIGHT_RANGE {
                forcesum += -(dif_vector.normalize() * BOID_SIGHT_RANGE - dif_vector);
            }
        }

        //Boid loneliness
        forcesum += 2. * (closest_boid - transform1.translation);

        //First force application
        if forcesum.length() > 0.1 {
            commands.entity(entity).insert(Force(forcesum));
        }
    }
}

pub fn sympathy_force_calc(
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
                && (transform2.translation - info.0).length() > SEPARATION_CIRCLE_RADIUS
            {
                let force = force_map.entry(entity).or_insert(force2.0);
                *force += force2.0 * 5.;
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

pub fn turn_boid(
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

pub fn move_boid(mut boids: Query<&mut Transform, With<Boid>>, timer: Res<Time>) {
    for mut transform in &mut boids {
        let direction = transform.local_x();
        transform.translation += direction * BOID_SPEED * timer.delta_seconds();
    }
}
