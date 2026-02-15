use crate::components::*;
use bevy::prelude::*;

const CUBE_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub fn update_cube(mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.translation = CUBE_POSITION;
    }
}
