use crate::ecs::{query::query, world::World};
use n64_math::{const_vec2, Vec2};

use super::movable::Movable;

static ENEMY_WAYPOINT: [Vec2; 4] = [
    const_vec2!([0.4, 0.4]),
    const_vec2!([0.6, 0.4]),
    const_vec2!([0.6, 0.6]),
    const_vec2!([0.4, 0.6]),
];

pub struct WaypointAi {
    pub waypoint: usize,
    pub waypoint_step: f32,
}

pub fn update(world: &mut World, dt: f32) {
    for (_e, ai, movable) in query::<(WaypointAi, Movable)>(&mut world.components) {
        if ai.waypoint_step >= 1.0 {
            ai.waypoint_step -= 1.0;
            ai.waypoint += 1;
            if ai.waypoint >= ENEMY_WAYPOINT.len() {
                ai.waypoint = 0;
            }
        }

        let a_waypoint = (ai.waypoint + 1) % ENEMY_WAYPOINT.len();
        let speed_a = ENEMY_WAYPOINT[a_waypoint] - ENEMY_WAYPOINT[ai.waypoint];
        let b_waypoint = (a_waypoint + 1) % ENEMY_WAYPOINT.len();
        let speed_b = ENEMY_WAYPOINT[b_waypoint] - ENEMY_WAYPOINT[a_waypoint];

        movable.speed = (1.0 - ai.waypoint_step) * speed_a + ai.waypoint_step * speed_b;
        ai.waypoint_step += dt;
    }
}
