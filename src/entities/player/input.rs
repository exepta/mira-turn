use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use crate::entities::player::PlayerWorldCamera;
use crate::entities::WorldPlayer;
use crate::events::player_events::PlayerActionEvent;
use crate::manager::GameState;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_keyboard_input, update_movement).run_if(in_state(GameState::InGame)));
    }
}

fn fetch_keyboard_input(mut input_event_writer: EventWriter<PlayerActionEvent>,
                        keyboard: Res<ButtonInput<KeyCode>>,
                        camera_query: Query<&Transform, With<PlayerWorldCamera>>,
                        mut player_query: Query<&mut WorldPlayer, With<WorldPlayer>>,
                        time: Res<Time>,
) {
    for mut player in player_query.iter_mut() {
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
                if keyboard.pressed(left_key) || keyboard.pressed(right_key) ||
                    keyboard.pressed(forward_key) || keyboard.pressed(backward_key) {
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
}

fn update_movement(time: Res<Time>,
                   mut input_event_reader: EventReader<PlayerActionEvent>,
                   mut player_query: Query<(&mut Transform, &mut Velocity, &mut WorldPlayer)>
) {
    for event in input_event_reader.read() {
        for (mut transform, mut velocity, mut player) in player_query.iter_mut() {
            match event {
                PlayerActionEvent::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        let movement_speed = (player.walk_speed * 100.0) * time.delta_secs();
                        velocity.linvel = Vec3::new(flat_direction.x * movement_speed, velocity.linvel.y, flat_direction.z * movement_speed);
                        velocity.angvel = Vec3::ZERO;
                    }
                }

                PlayerActionEvent::Sprinting(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let current_forward = transform.forward().as_vec3();

                        if current_forward.dot(flat_direction) < 0.99 {
                            let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                            transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        }

                        let movement_speed = (player.sprinting_speed * 100.0) * time.delta_secs();
                        velocity.linvel = Vec3::new(flat_direction.x * movement_speed, velocity.linvel.y, flat_direction.z * movement_speed);
                        velocity.angvel = Vec3::ZERO;
                    }
                }

                PlayerActionEvent::Idle => {
                    velocity.linvel = Vec3::new(0.0, velocity.linvel.y, 0.0);
                    velocity.angvel = Vec3::ZERO;
                }
            }
        }
    }
}