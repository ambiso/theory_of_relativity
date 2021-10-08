use bevy::prelude::*;

use crate::game_object::*;

pub struct PhysicsPlugin;
pub struct PhysicsObject;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(physics.system());
    }
}

fn physics(query: Query<&Pos, With<PhysicsObject>>) {
    for pos in query.iter() {
        println!("Wow there's an entity at {}", pos.0);
    }
}
