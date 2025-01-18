use bevy::prelude::*;
use crate::manager::GameState;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_game_floor);
    }
}

fn create_game_floor(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))))
        .insert(MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))))
        .insert(Name::new("Floor"));
}