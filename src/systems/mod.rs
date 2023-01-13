mod player_input;
mod map_render;
mod entity_render;
mod random_move;
mod turns;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod gather;
mod chasing;

use crate::prelude::*;

// This is where player sits and decides what to do. Waiting for player input
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltip_system())
        .build()
}

// This is executed right after player finilized the input
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
        .add_system(gather::gather_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltip_system())
        .add_system(turns::turn_system())
        .build()
}

// This is world's turn. Do stuff here like grow, heal respawn e.t.c.
pub fn build_world_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
        .add_system(random_move::random_move_system())
        .add_system(chasing::chase_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltip_system())
        .add_system(turns::turn_system())
        .build()
}