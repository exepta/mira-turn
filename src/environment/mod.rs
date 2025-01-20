use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{AsyncCollider, RigidBody, TriMeshFlags};
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
        .insert(Name::new("Floor"))
        .insert(RigidBody::Fixed)
        .insert(AsyncCollider(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .build(),
    ));
}