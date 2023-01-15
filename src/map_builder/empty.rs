use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            resource_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(rng.range(1, MAP_WIDTH), rng.range(1, MAP_HEIGHT));
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.monster_spawns.push(
                Point::new(
                    rng.range(1, MAP_WIDTH),
                    rng.range(1, MAP_HEIGHT),
                )
            );
        }

        for _ in 0..50 {
            mb.resource_spawns.push(
                Point::new(
                    rng.range(1, MAP_WIDTH),
                    rng.range(1, MAP_HEIGHT),
                )
            );
        }

        mb
    }
}