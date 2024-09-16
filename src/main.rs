use avian3d::prelude::*;
use bevy::prelude::*;
use dice::{roll_dices, spawn_dices, teleport_fallen_dices};
use table::setup;
use ui::UiPlugin;

mod camera;
mod combination;
mod dice;
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
            camera::FlyCamPlugin,
        ))
        .add_systems(Startup, (setup, spawn_dices))
        .add_systems(Update, (roll_dices, teleport_fallen_dices))
        .run();
}
