use bevy::prelude::*;

#[derive(Resource)]
pub struct RetriesLeft(pub u8);

impl Default for RetriesLeft {
    fn default() -> Self {
        Self(3)
    }
}

#[derive(Resource, Default)]
pub struct CanSkipTurn(pub bool);

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Setup,
    NPCRolling,
    PlayerRolling,
    //TODO: Shopping,
}

pub fn setup_game_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::NPCRolling);
}
