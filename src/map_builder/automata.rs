use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::empty();

        self.random_noise_map(rng, &mut mb.map);

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor
            } else {
                *t = TileType::Wall
            }
        });
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;

        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[Map::map_index(x + ix, y + iy)] == TileType::Wall {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..MAP_HEIGHT - 1 {
            for x in 1..MAP_WIDTH - 1 {
                let neightbours = self.count_neighbours(x, y, map);
                let idx = Map::map_index(x, y);
                if neightbours > 4 || neightbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_Start(&self, map: &Map) -> Point {
        let center = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        let closes_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx))))
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closes_point)
    }
}