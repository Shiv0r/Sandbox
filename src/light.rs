use bevy::prelude::*;

const AMBIENT_DAYLIGHT: f32 = 10000.0;

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1., 0.96, 0.95),
            shadows_enabled: true,
            illuminance: AMBIENT_DAYLIGHT,
            ..Default::default()
        },
        Transform::default().looking_at(Vec3::new(-1., -1., -1.), Vec3::Y),
    ));
}
