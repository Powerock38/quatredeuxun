use bevy::prelude::*;

use crate::{
    combination::Combination,
    game::{CanSkipTurn, GameState, ThrowsLeft},
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct SkipTurnButton;

#[derive(Component)]
struct ThrowsLeftText;

fn setup_ui(mut commands: Commands) {
    commands.observe(on_display_score);

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                ScoreText,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
            ));

            c.spawn((
                SkipTurnButton,
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(5.0)),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
            ))
            .with_children(|c| {
                c.spawn(TextBundle::from_section(
                    "Stop there",
                    TextStyle {
                        font_size: 40.0,
                        ..default()
                    },
                ));
            });

            c.spawn((
                ThrowsLeftText,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
            ));
        });
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
            "To beat: {}.\nYou scored: {}\n{}",
            score.npc,
            player,
            if *wins { "You win!" } else { "You lose!" }
        ),
        None => format!("To beat: {}", score.npc),
    }
}

fn update_skip_turn_button(
    mut q_btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SkipTurnButton>),
    >,
    mut throws: ResMut<ThrowsLeft>,
) {
    for (interaction, mut color) in &mut q_btn {
        match *interaction {
            Interaction::Pressed => {
                throws.0 = 0;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn update_throws(mut query: Query<&mut Text, With<ThrowsLeftText>>, throws: Res<ThrowsLeft>) {
    if throws.is_added() || throws.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Throws left: {}", throws.0);
    }
}

pub fn apply_font(asset_server: Res<AssetServer>, mut query: Query<&mut Text, Added<Text>>) {
    for mut text in &mut query {
        for section in &mut text.sections {
            section.style.font = asset_server.load("JqkasWild.ttf");
        }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                apply_font,
                update_throws,
                update_skip_turn_button
                    .run_if(in_state(GameState::PlayerRolling))
                    .run_if(|can: Res<CanSkipTurn>| can.0),
            ),
        );
    }
}
