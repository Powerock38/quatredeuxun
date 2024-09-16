use bevy::prelude::*;

use crate::combination::Combination;

#[derive(Resource)]
pub struct LastCombination(pub Combination);

#[derive(Component)]
struct LastCombinationText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        LastCombinationText,
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 100.0,
                ..default()
            },
        ),
    ));
}

fn update_ui(
    mut query: Query<&mut Text, With<LastCombinationText>>,
    last_combination: Option<Res<LastCombination>>,
) {
    if let Some(last_combination) = last_combination {
        if last_combination.is_added() {
            for mut text in &mut query {
                text.sections[0].value = format!("{}", last_combination.0);
            }
        }
    } else {
        for mut text in &mut query {
            text.sections[0].value = String::new();
        }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
