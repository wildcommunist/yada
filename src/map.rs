use crate::prelude::*;

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Portal,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    // Vectors are indexed on single dimension
    /*
    We need a way to transform map coordinates [x][y] to vector location.
    This process is called striding.
     */
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn map_index(x: i32, y: i32) -> usize {
        ((y * MAP_WIDTH) + x) as usize
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.is_in_map_bounds(point) {
            None
        } else {
            Some(Self::map_index(point.x, point.y))
        }
    }

    // This function checks if given point is within the map boundary
    pub fn is_in_map_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    // Check if we can enter the tile. For now it just checks if the tile is floor
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.is_in_map_bounds(point) &&
            (self.tiles[Self::map_index(point.x, point.y)] == TileType::Floor
                || self.tiles[Self::map_index(point.x, point.y)] == TileType::Portal)
    }

    // Determine valid exists from given location. So tile from which you are moving, can u go to the delta?
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.is_in_map_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.is_in_map_bounds(pos)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Left
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0)) // This is the cost of the move, the lower the value, more preferable it is
        }

        // Right
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }

        // Down
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }

        // Up
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2),
            )
    }
}