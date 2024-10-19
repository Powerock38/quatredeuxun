use bevy::prelude::*;

use crate::{combination::Combination, game::ThrowsLeft};

#[derive(Component)]
struct ThrowsLeftText;

#[derive(Component)]
struct ScoreText;

fn setup_ui(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.observe(on_display_score);

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                ThrowsLeftText,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 50.0,
                        font: assets_server.load("JqkasWild.ttf"),
                        ..default()
                    },
                ),
            ));

            //TODO: button to remove all ThrowsLeft

            //FIXME: doesnt show up
            c.spawn((
                ScoreText,
                TextBundle::from_section(
                    "zzz",
                    TextStyle {
                        font_size: 50.0,
                        font: assets_server.load(
                            "JqkasWild.ttf
                    ",
                        ),
                        ..default()
                    },
                ),
            ));
        });
}

fn update_throws(mut query: Query<&mut Text, With<ThrowsLeftText>>, throws: Res<ThrowsLeft>) {
    if throws.is_added() || throws.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Throws left: {}", throws.0);
    }
}

#[derive(Event)]
pub struct DisplayScore {
    npc: Combination,
    player: Option<(Combination, bool)>, // (combination, wins) if player threw dices
}

impl DisplayScore {
    pub fn npc(npc: Combination) -> Self {
        Self { npc, player: None }
    }

    pub fn player(npc: Combination, player: Combination, wins: bool) -> Self {
        Self {
            npc,
            player: Some((player, wins)),
        }
    }
}

fn on_display_score(trigger: Trigger<DisplayScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    let score = trigger.event();
    let mut text = query.single_mut();

    text.sections[0].value = match &score.player {
        Some((player, wins)) => format!(
            "To beat: {}. You scored: {}\n{}",
            score.npc,
            player,
            if *wins { "You win!" } else { "You lose!" }
        ),
        None => format!("To beat: {}", score.npc),
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_throws);
    }
}
