use crate::map_builder::MapArchitect;
use crate::prelude::*;

// How many times each worker should `dig` before expiring
const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
//Percentage of the map should be available
const PERCENT_AVAILABLE: usize = 75;
const DESIRED_FLOOR: usize = NUM_TILES / 100 * PERCENT_AVAILABLE;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();

        mb.fill(TileType::Wall);
        let center = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);

        while mb.map.tiles.iter()
            .filter(|tt| **tt == TileType::Floor).count() < DESIRED_FLOOR {
            self.drunkard(
                &Point::new(
                    rng.range(0, MAP_WIDTH),
                    rng.range(0, MAP_HEIGHT),
                ),
                rng,
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                MAP_WIDTH, MAP_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );

            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        let mut drunkard_pos = *start;
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y -= 1,
            }

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}