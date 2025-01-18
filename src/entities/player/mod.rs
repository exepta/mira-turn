use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use crate::entities::WorldPlayer;
use crate::manager::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_world_player);
        app.add_systems(OnEnter(GameState::InGame), create_player_camera);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerCamera;

fn create_world_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/player/placeholder.glb"))))
        .insert(Name::new("WorldPlayer"))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(ThirdPersonCameraTarget)
        .insert(WorldPlayer::default());
}

fn create_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        PlayerCamera,
        ThirdPersonCamera {
            sensitivity: Vec2::new(2.0, 2.0),
            zoom: Zoom::new(3.5, 12.75),
            cursor_lock_key: KeyCode::Escape,
            offset: Offset::new(0.0, 0.8),
            offset_enabled: true,
            ..default()
        },
        Bloom::default(),
        DistanceFog {
            color: Color::srgb(0.3, 0.3, 0.32),
            falloff: FogFalloff::Linear {
                start: 500.0,
                end: 600.0
            },
            ..default()
        }
    ));
}
