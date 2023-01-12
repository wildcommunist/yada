mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
