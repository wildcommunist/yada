mod player_input;
mod map_render;
mod entity_render;
mod random_move;
mod turns;
mod movement;
mod hud;
mod tooltips;
mod combat;

use crate::prelude::*;

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

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
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

pub fn build_world_scheduler() -> Schedule {
    Schedule::builder()
        // Add systems here
        .add_system(random_move::random_move_system())
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