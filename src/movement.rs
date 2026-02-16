use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rotation);
    }
}

fn update_rotation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let angle_degree = velocity.value.z * time.delta_secs();
        let angle_radians = angle_degree.to_radians();

        let rotation = Quat::from_rotation_z(angle_radians);
        transform.rotation *= rotation;
    }
}
