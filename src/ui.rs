use bevy::prelude::*;

use crate::{
    combination::Combination,
    game::{ToBeat, Tries},
};

#[derive(Resource)]
pub struct LastCombination(pub Combination);

#[derive(Component)]
struct LastCombinationText;

#[derive(Component)]
struct ToBeatText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                ToBeatText,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
            ));

            c.spawn((
                LastCombinationText,
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

fn update_ui(
    mut q_last_combi_text: Query<&mut Text, With<LastCombinationText>>,
    mut q_to_beat_text: Query<&mut Text, (With<ToBeatText>, Without<LastCombinationText>)>,
    last_combination: Option<Res<LastCombination>>,
    to_beat: Res<ToBeat>,
    tries: Res<Tries>,
) {
    let mut last_combi_text = q_last_combi_text.single_mut();

    if let Some(last_combination) = last_combination {
        if last_combination.is_added() {
            last_combi_text.sections[0].value = format!("{}", last_combination.0);

            if tries.0 == to_beat.tries {
                if last_combination.0 >= to_beat.combination {
                    last_combi_text.sections[0].value += " - You win!";
                } else {
                    last_combi_text.sections[0].value += " - You lose!";
                }
            }
        }
    } else {
        last_combi_text.sections[0].value = String::new();
    }

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
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
