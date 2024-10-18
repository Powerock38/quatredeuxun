use bevy::prelude::*;

use crate::{combination::LastCombination, game::Tries};

#[derive(Component)]
struct LastCombinationText(Timer);

impl LastCombinationText {
    fn new() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

#[derive(Component)]
struct ToBeatText;

fn setup_ui(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        ToBeatText,
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 50.0,
                font: assets_server.load("JqkasWild.ttf"),
                ..default()
            },
        ),
    ));
}

fn spawn_last_combination(
    mut commands: Commands,
    last_combination: Option<Res<LastCombination>>,
    assets_server: Res<AssetServer>,
    tries: Res<Tries>,
) {
    if let Some(last_combination) = last_combination {
        if last_combination.is_added() || last_combination.is_changed() {
            commands.spawn((
                LastCombinationText::new(),
                TextBundle::from_section(
                    format!(
                        "{}\n{}",
                        last_combination.player,
                        if tries.0 == 0 {
                            if last_combination.wins() {
                                "You win!"
                            } else {
                                "You lose!"
                            }
                        } else {
                            ""
                        }
                    ),
                    TextStyle {
                        font_size: 50.0,
                        font: assets_server.load("JqkasWild.ttf"),
                        ..default()
                    },
                )
                .with_style(Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            ));
        }
    }
}

fn update_last_combination(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LastCombinationText)>,
    time: Res<Time>,
) {
    for (entity, mut last_combination_text) in &mut query {
        if last_combination_text.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, (spawn_last_combination, update_last_combination));
    }
}
