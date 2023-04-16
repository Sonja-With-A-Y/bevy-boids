//Imports
use std::collections::HashMap;
use std::time::Duration;

use bevy::{math::f32::Vec3, pbr::prelude::*, prelude::*};

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod assets;
mod boids;
mod consts;
mod seeds;
mod setup;

pub use assets::*;
pub use boids::*;
pub use consts::*;
pub use seeds::*;
pub use setup::*;

//Main
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_startup_system(asset_loading.in_base_set(StartupSet::PreStartup))
        .add_startup_system(setup)
        .add_systems(
            (
                boid_force_calc,
                sympathy_force_calc,
                turn_boid,
                move_boid,
                drop_seeds,
                delete_seed,
            )
                .chain(),
        )
        .run();
}
