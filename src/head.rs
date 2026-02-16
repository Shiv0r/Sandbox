use crate::movement::Velocity;
use bevy::prelude::*;

fn starting_rotation() -> Quat {
    Quat::from_rotation_z(0.0_f32.to_radians())
}
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 15.);

#[derive(Bundle, Default)]
struct HeadBundle {
    velocity: Velocity,
    model: SceneRoot,
    transform: Transform,
}

pub struct HeadPlugin;
impl Plugin for HeadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_head);
    }
}

fn spawn_head(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(HeadBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneRoot(asset_server.load("PBR_HeadTest.gltf#Scene0")),
        transform: Transform::from_rotation(starting_rotation()),
        ..default()
    });
    commands.spawn(AmbientLight {
        color: Color::hsl(300., 0.7, 0.85),
        brightness: 250.0,
        affects_lightmapped_meshes: true,
    });
}
