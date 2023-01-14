mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const MAP_WIDTH: i32 = 150;
    pub const MAP_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

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
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_item(&mut ecs, map_builder.amulet_start);

        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|rp| spawn_monster(&mut ecs, &mut rng, rp));

        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|rp| spawn_resource(&mut ecs, &mut rng, Point::new(rp.x + 1, rp.y + 1)));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

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
            self.ecs = World::default();
            self.resources = Resources::default();
            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::new(&mut rng);

            spawn_player(&mut self.ecs, map_builder.player_start);
            spawn_item(&mut self.ecs, map_builder.amulet_start);

            map_builder.rooms
                .iter()
                .skip(rng.range(0, 3))
                .map(|r| r.center())
                .for_each(|rp| spawn_monster(&mut self.ecs, &mut rng, rp));

            map_builder.rooms
                .iter()
                .skip(rng.range(0, 6))
                .map(|r| r.center())
                .for_each(|rp| spawn_resource(&mut self.ecs, &mut rng, Point::new(rp.x + 1, rp.y + 1)));

            self.resources.insert(map_builder.map);
            self.resources.insert(Camera::new(map_builder.player_start));
            self.resources.insert(TurnState::AwaitingInput);
        }
    }
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

        let current_state = self.resources.get::<TurnState>().unwrap().clone(); //I dont like this. Danger!
        match current_state {
            TurnState::AwaitingInput => self.input_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::WorldTurn => self.world_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Yet Another Dungeon Adventure")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
