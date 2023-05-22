use crate::*;

pub fn setup(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    //Ambient light
    commands.insert_resource(AmbientLight {
        brightness: 1.5,
        ..default()
    });

    //Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 25000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 1., 2.),
            rotation: Quat::from_rotation_y(70_f32.to_radians()),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            maximum_distance: 500.,
            first_cascade_far_bound: 10.,
            ..default()
        }
        .into(),
        ..default()
    });

    //Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., -300., 80.0).looking_at(Vec3::ZERO, Vec3::Z),
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
                transform: Transform::from_translation(Vec3::new(j*3. - 100., 0., 0.))
                    .with_rotation(Quat::from_rotation_z((j * 30.0_f32 + 1.).to_radians()))
                    .with_scale(Vec3 {
                        x: BOID_SCALE,
                        y: BOID_SCALE,
                        z: BOID_SCALE,
                    }),
                ..Default::default()
            },
            Boid,
        ));
    }

    //Start see timer
    commands.insert_resource(SeedTimer {
        timer: Timer::new(Duration::from_secs(SEED_SPAWN_RATE), TimerMode::Repeating),
    })
}
