use avian3d::prelude::*;
use bevy::prelude::*;
use dice::analyze_dices;
use game::{setup_game_state, CanSkipTurn, GameState, RetriesLeft};
use npc::{reroll_fallen_npc_dices, roll_npc_dices, spawn_npc_dices};
use player::{
    click_spawns_raycast, manage_selected_dice_animation, pickup_all_player_dices,
    pickup_fallen_dices, raycast_dices, spawn_camera, spawn_player_dices,
};
use table::{punch_table, setup};
use ui::UiPlugin;

mod combination;
mod dice;
mod flycam;
mod game;
mod npc;
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
            //bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ))
        .add_systems(
            Startup,
            (setup, spawn_camera, spawn_player_dices, spawn_npc_dices),
        )
        .add_systems(
            Update,
            (
                setup_game_state.run_if(in_state(GameState::Setup)),
                (pickup_fallen_dices, click_spawns_raycast, raycast_dices)
                    .run_if(in_state(GameState::PlayerRolling)),
                analyze_dices,
                manage_selected_dice_animation,
                punch_table,
                reroll_fallen_npc_dices,
            ),
        )
        .add_systems(
            OnEnter(GameState::NPCRolling),
            (pickup_all_player_dices, roll_npc_dices),
        )
        .init_state::<GameState>()
        .init_resource::<RetriesLeft>()
        .init_resource::<CanSkipTurn>()
        .run();
}
