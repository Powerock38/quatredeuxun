use bevy::prelude::*;

#[derive(Resource)]
pub struct ThrowsLeft(pub u8);

impl Default for ThrowsLeft {
    fn default() -> Self {
        Self(6)
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
