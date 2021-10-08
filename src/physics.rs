use bevy::prelude::*;

use crate::game_object::*;

pub struct PhysicsPlugin;

#[derive(Default)]
pub struct PhysicsObject {
    momentum: Vec3,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(physics_update.system());
    }
}

fn physics_update(
    time: Res<Time>,
    mut query: QuerySet<(
        Query<(&Position, &Transform), With<PhysicsObject>>,
        Query<(&mut Position, &mut Transform), With<PhysicsObject>>,
    )>,
    mut q2: Query<&mut PhysicsObject, With<Position>>,
) {
    for ((my_pos, _), mut physics_object) in query.q0().iter().zip(q2.iter_mut()) {
        let mut influences = Vec3::ZERO;
        for (other_pos, _) in query.q0().iter() {
            let delta = other_pos.0.translation - my_pos.0.translation;
            let len = delta.length();
            if len > 100. {
                influences += delta / len / len / len;
            }
        }
        influences *= time.delta_seconds();
        influences *= 10000.;
        physics_object.momentum += influences;
    }
    for ((mut position, mut transform), physics_object) in
        query.q1_mut().iter_mut().zip(q2.iter_mut())
    {
        position.0.translation += physics_object.momentum;
        *transform = position.0;
        println!("Delta: {}", physics_object.momentum);
    }
}
