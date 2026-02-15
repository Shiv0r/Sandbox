mod components;
mod entities;
mod systems;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::entities::{spawn_camera, spawn_cube, spawn_directional_light};
use crate::systems::update_cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (spawn_camera, spawn_cube, spawn_directional_light))
        .add_systems(Update, update_cube)
        .run();
}
