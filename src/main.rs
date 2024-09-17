use avian3d::prelude::*;
use bevy::prelude::*;
use dice::{
    analyze_dices, filter_collisions, highlight_selected_dice, pickup_fallen_dices, raycast_dices,
    spawn_dices,
};
use player::{click_spawns_raycast, spawn_camera};
use table::setup;
use ui::UiPlugin;

mod combination;
mod dice;
mod flycam;
mod player;
mod table;
mod ui;

fn main() {
    App::new()
        // Enable physics
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }),
            PhysicsPlugins::default(),
            UiPlugin,
            flycam::FlyCamPlugin,
        ))
        .add_systems(Startup, (setup, spawn_camera, spawn_dices))
        .add_systems(
            Update,
            (
                pickup_fallen_dices,
                click_spawns_raycast,
                raycast_dices,
                analyze_dices,
                highlight_selected_dice,
            ),
        )
        .add_systems(PostProcessCollisions, filter_collisions)
        .run();
}
