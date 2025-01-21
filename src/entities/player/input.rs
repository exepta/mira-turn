use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterController;
use crate::entities::player::PlayerWorldCamera;
use crate::entities::WorldPlayer;
use crate::events::player_events::PlayerActionEvent;
use crate::manager::GameState;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (detect_input, update_system).run_if(in_state(GameState::InGame)));
    }
}

fn detect_input(mut input_event_writer: EventWriter<PlayerActionEvent>,
                keyboard: Res<ButtonInput<KeyCode>>,
                camera_query: Query<&Transform, With<PlayerWorldCamera>>
) {
    if let Ok(cam_transform) = camera_query.get_single() {
        let forward_key = KeyCode::KeyW;
        let backward_key = KeyCode::KeyS;
        let left_key = KeyCode::KeyA;
        let right_key = KeyCode::KeyD;

        let sprinting_key = KeyCode::ShiftLeft;

        let mut direction = Vec3::ZERO;
        if keyboard.pressed(forward_key) {
            direction += Vec3::new(cam_transform.forward().x, direction.y, cam_transform.forward().z);
        }

        if keyboard.pressed(backward_key) {
            direction += Vec3::new(cam_transform.back().x, direction.y, cam_transform.back().z);
        }

        if keyboard.pressed(left_key) {
            direction += cam_transform.left().as_vec3();
        }
        if keyboard.pressed(right_key) {
            direction += cam_transform.right().as_vec3();
        }

        if direction.length_squared() > 0.0 {
            let normalized_direction = direction.normalize();
            if keyboard.pressed(left_key) || keyboard.pressed(right_key)
                || keyboard.pressed(backward_key) || keyboard.pressed(forward_key) {
                input_event_writer.send(PlayerActionEvent::Move(normalized_direction));
            } else {
                input_event_writer.send(PlayerActionEvent::Idle);
            }
        } else {
            input_event_writer.send(PlayerActionEvent::Idle);
        }

        if keyboard.pressed(sprinting_key) {
            input_event_writer.send(PlayerActionEvent::Sprinting(direction.normalize()));
        }
    }
}

fn update_system(time: Res<Time>,
                 mut controllers: Query<(&mut KinematicCharacterController, &WorldPlayer), With<WorldPlayer>>,
                 mut input_event_reader: EventReader<PlayerActionEvent>,) {
    for event in input_event_reader.read() {
        for (mut controller, world_player) in controllers.iter_mut() {
            match event {
                PlayerActionEvent::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let movement_speed = world_player.walk_speed * time.delta_secs();
                        controller.translation = Some(Vec3::new(direction.x * movement_speed, direction.y, direction.z * movement_speed));
                    }
                },
                PlayerActionEvent::Sprinting(direction) => {
                    if direction.length_squared() > 0.0 {
                        let movement_speed = world_player.sprinting_speed * time.delta_secs();
                        controller.translation = Some(Vec3::new(direction.x * movement_speed, direction.y, direction.z * movement_speed));
                    }
                },
                PlayerActionEvent::Idle => {
                    controller.translation = None;
                }
            }
        }
    }
}