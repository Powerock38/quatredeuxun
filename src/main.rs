use avian3d::prelude::*;
use bevy::prelude::*;
use dice::{
    analyze_dices, filter_collisions_in_hand, manage_selected_dice_animation, pickup_fallen_dices,
    raycast_dices, spawn_dices,
};
use game::{ToBeat, Tries};
use player::{click_spawns_raycast, spawn_camera};
use table::{punch_table, setup};
use ui::UiPlugin;

mod combination;
mod dice;
mod flycam;
mod game;
mod player;
mod table;
mod ui;

fn main() {
    App::new()
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
                manage_selected_dice_animation,
                punch_table,
            ),
        )
        .add_systems(PostProcessCollisions, filter_collisions_in_hand)
        .insert_resource(ToBeat::roll())
        .init_resource::<Tries>()
        .run();
}
