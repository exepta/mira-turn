mod input;

use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};
use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use crate::entities::player::input::PlayerInputPlugin;
use crate::entities::WorldPlayer;
use crate::manager::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerInputPlugin);
        app.add_systems(OnEnter(GameState::InGame), create_world_player);
        app.add_systems(OnEnter(GameState::InGame), create_player_camera);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerWorldCamera;

fn create_world_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/player/player_idle_2.glb"))))
        .insert(Name::new("WorldPlayer"))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(ThirdPersonCameraTarget)
        .insert(WorldPlayer::default())
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::capsule(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2))
        .insert(Damping {
            angular_damping: 0.5,
            linear_damping: 0.2
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z);
}

fn create_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        PlayerWorldCamera,
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
