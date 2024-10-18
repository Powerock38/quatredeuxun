use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Tries(pub u8); //TODO tries

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Setup,
    NPCRolling,
    PlayerRolling,
}

pub fn setup_game_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::NPCRolling);
}
