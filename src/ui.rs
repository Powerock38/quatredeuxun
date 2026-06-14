use bevy::prelude::*;

use crate::{
    combination::Combination,
    game::{CanSkipTurn, GameState, RetriesLeft},
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
#[require(Text::default(), TextFont::from_font_size(50.0))]
struct ScoreText;

#[derive(Component)]
#[require(
    Button,
    BackgroundColor(NORMAL_BUTTON),
    Node {
        padding: UiRect::all(Val::Px(5.0)),
        height: Val::Px(50.0),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    },
    BorderColor::all(Color::WHITE),
)]
struct SkipTurnButton;

#[derive(Component)]
#[require(Text::default(), TextFont::from_font_size(50.0))]
struct RetriesLeftText;

fn setup_ui(mut commands: Commands) {
    commands.add_observer(on_display_score);

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|c| {
            c.spawn(ScoreText);

            c.spawn(SkipTurnButton).with_children(|c| {
                c.spawn((Text("Stop there".into()), TextFont::from_font_size(40.0)));
            });

            c.spawn(RetriesLeftText);
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

fn on_display_score(trigger: On<DisplayScore>, mut text: Single<&mut Text, With<ScoreText>>) {
    let score = trigger.event();

    text.0 = match &score.player {
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
    mut retries: ResMut<RetriesLeft>,
) {
    for (interaction, mut color) in &mut q_btn {
        match *interaction {
            Interaction::Pressed => {
                retries.0 = 0;
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

fn update_retries(mut text: Single<&mut Text, With<RetriesLeftText>>, retries: Res<RetriesLeft>) {
    if retries.is_added() || retries.is_changed() {
        text.0 = format!("Retries left: {}", retries.0);
    }
}

pub fn apply_font(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut TextFont, Added<TextFont>>,
) {
    for mut font in &mut query {
        font.font = asset_server.load("JqkasWild.ttf");
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                apply_font,
                update_retries,
                update_skip_turn_button
                    .run_if(in_state(GameState::PlayerRolling))
                    .run_if(|can: Res<CanSkipTurn>| can.0),
            ),
        );
    }
}
