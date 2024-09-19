use bevy::prelude::*;

use crate::{
    combination::LastCombination,
    game::{ToBeat, Tries},
};

#[derive(Component)]
struct LastCombinationText(Timer);

impl LastCombinationText {
    fn new() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

#[derive(Component)]
struct ToBeatText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        ToBeatText,
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        ),
    ));
}

fn spawn_last_combination(
    mut commands: Commands,
    last_combination: Option<Res<LastCombination>>,
    tries: Res<Tries>,
) {
    if let Some(last_combination) = last_combination {
        if last_combination.is_added() || last_combination.is_changed() {
            commands.spawn((
                LastCombinationText::new(),
                TextBundle::from_section(
                    format!(
                        "{}\n{}",
                        last_combination.combination,
                        if tries.0 == 0 {
                            if last_combination.enough {
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

fn update_to_beat(
    mut q_to_beat_text: Query<&mut Text, (With<ToBeatText>, Without<LastCombinationText>)>,
    to_beat: Res<ToBeat>,
    tries: Res<Tries>,
) {
    let mut to_beat_text = q_to_beat_text.single_mut();

    if to_beat.is_added() || to_beat.is_changed() || tries.is_added() || tries.is_changed() {
        to_beat_text.sections[0].value = format!(
            "Beat {} ({}/{})",
            to_beat.combination, tries.0, to_beat.tries
        );
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                update_to_beat,
                spawn_last_combination,
                update_last_combination,
            ),
        );
    }
}
