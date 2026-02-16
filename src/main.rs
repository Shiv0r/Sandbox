mod camera;
mod debug;
mod head;
mod light;
mod movement;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::head::HeadPlugin;
use crate::light::LightPlugin;
use crate::movement::MovementPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(200., 0.5, 0.35)))
        //Startup
        .add_plugins(HeadPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        //Update
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
