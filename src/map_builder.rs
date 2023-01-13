use crate::prelude::*;

const NUM_ROOMS: usize = 40;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_room(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut()
            .for_each(|t| *t = tile);
    }

    fn build_random_room(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, MAP_WIDTH - 10),
                rng.range(1, MAP_HEIGHT - 10),
                rng.range(3, 50),
                rng.range(3, 50),
            );

            let mut overlap = false;
            for r in &self.rooms {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                // we have no overlap, add room to the map
                room.for_each(|p| {
                    if p.x > 0 && p.x < MAP_WIDTH && p.y > 0 && p.y < MAP_HEIGHT {
                        // the room is inside bounds of the screen
                        let idx = Map::map_index(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            // we skip first room as well reference it in the loop (previous room)
            let prev_room_center = rooms[i - 1].center();
            let current_room_center = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev_room_center.x, current_room_center.x, prev_room_center.y);
                self.apply_vertical_tunnel(prev_room_center.y, current_room_center.y, current_room_center.x);
            } else {
                self.apply_vertical_tunnel(prev_room_center.y, current_room_center.y, prev_room_center.x);
                self.apply_horizontal_tunnel(prev_room_center.x, current_room_center.x, current_room_center.y);
            }
        }
    }
}