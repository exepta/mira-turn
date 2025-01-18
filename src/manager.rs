use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_plugins((EntitiesPlugin, EnvironmentPlugin));
    }
}

#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    SplashScreen,
    TitleScreen,
    AccountScreen,
    #[default]
    InGame
}