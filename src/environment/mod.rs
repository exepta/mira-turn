use bevy::prelude::*;
use crate::manager::GameState;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Launching), create_game_floor);
    }
}

fn create_game_floor(mut commands: Commands) {

}