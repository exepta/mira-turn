mod player;

use bevy::prelude::*;
use crate::entities::player::PlayerPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AccountPlayer>();
        app.register_type::<WorldPlayer>();
        app.register_type::<Character>();
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AccountPlayer {
    pub account_level: usize,
    pub name: String,
    pub email: String,
    pub uid: usize,
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldPlayer {
    pub actions_points: usize,
    pub walk_speed: f32,
    pub sprinting_speed: f32,
}

impl Default for WorldPlayer {
    fn default() -> Self {
        Self {
            actions_points: 3,
            walk_speed: 3.0,
            sprinting_speed: 4.5,
        }
    }
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Character {
    pub hp: usize,
    pub attack: usize,
    pub defense: usize,
    pub speed: usize,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub energy_charge_rate: f32,
}