use bevy::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_rotation);
    }
}

fn print_rotation(query: Query<(Entity, &Transform)>) {
    for (enity, transform) in query.iter() {
        let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);
        info!(
            "Enitity {:?} rotation (radians): x={:.2}, y={:.2}, z={:.2}",
            enity, x, y, z
        )
    }
}
