use crate::map_builder::MapArchitect;
use crate::prelude::*;

pub struct RoomArchitect {}

impl MapArchitect for RoomArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();

        mb.fill(TileType::Wall);
        mb.build_random_room(rng);
        mb.build_corridors(rng);

        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();

        for room in mb.rooms.iter().skip(1) {
            if rng.range(0, 10) > 2 {
                mb.monster_spawns.push(Point::new(
                    rng.range(room.x1 + 1, room.x2 - 1),
                    rng.range(room.y1 + 1, room.y2 - 1),
                ))
            }


            if rng.range(0, 10) > 7 {
                mb.resource_spawns.push(Point::new(
                    rng.range(room.x1 + 1, room.x2 - 1),
                    rng.range(room.y1 + 1, room.y2 - 1),
                ))
            }
        }

        mb
    }
}