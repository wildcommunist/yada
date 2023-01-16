mod map;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const MAP_WIDTH: i32 = 100;
    pub const MAP_HEIGHT: i32 = 100;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::process::exit;
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    world_system: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        //spawn_item(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Portal;

        map_builder.monster_spawns
            .iter()
            .for_each(|monster_pos| spawn_monster(&mut ecs, &mut rng, *monster_pos));


        map_builder.resource_spawns
            .iter()
            .for_each(|monster_pos| spawn_loot_item(&mut ecs, &mut rng, *monster_pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_system: build_input_scheduler(),
            player_system: build_player_scheduler(),
            world_system: build_world_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your adventure has come to an end!");
        ctx.print_color_centered(4, WHITE, BLACK, "When your HP hits 0, you are going to have a bad time.");
        ctx.print_color_centered(6, YELLOW, BLACK, "Press [R] to restart, [Q] to exit.");

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset_game_state();
        }

        if let Some(VirtualKeyCode::Q) = ctx.key {
            exit(0);
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have done it, you have found the amulet of YALA");
        ctx.print_color_centered(4, WHITE, BLACK, "You return to your village, as a hero, wearing the amulet around your neck!");
        ctx.print_color_centered(6, YELLOW, BLACK, "Press [R] to restart, [Q] to exit.");

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset_game_state();
        }

        if let Some(VirtualKeyCode::Q) = ctx.key {
            exit(0);
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        //spawn_item(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Portal;

        map_builder.monster_spawns
            .iter()
            .for_each(|monster_pos| spawn_monster(&mut self.ecs, &mut rng, *monster_pos));


        map_builder.resource_spawns
            .iter()
            .for_each(|resource_pos| spawn_resource(&mut self.ecs, &mut rng, *resource_pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn advance_level(&mut self) {}
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0); //map layer
        ctx.cls();
        ctx.set_active_console(1); // player layer
        ctx.cls();
        ctx.set_active_console(2); // player layer
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self.resources.get::<TurnState>().unwrap(); //I dont like this. Danger!
        match current_state {
            TurnState::AwaitingInput => self.input_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::WorldTurn => self.world_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Yet Another Dungeon Adventure")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(48, 48)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Console 0
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //Console 1
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png") //Console2
        .build()?;

    main_loop(context, State::new())
}
