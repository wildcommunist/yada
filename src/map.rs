use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>, // Vectors are indexed on single dimension
    /*
    We need a way to transform map coordinates [x][y] to vector location.
    This process is called striding.
     */
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn map_index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.is_in_map_bounds(point) {
            None
        } else {
            Some(Self::map_index(point.x, point.y))
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.is_in_map_bounds(Point::new(x, y)) {
                    let idx = Self::map_index(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }

    // This function checks if given point is within the map boundary
    pub fn is_in_map_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Check if we can enter the tile. For now it just checks if the tile is floor
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.is_in_map_bounds(point) &&
            self.tiles[Self::map_index(point.x, point.y)] == TileType::Floor
    }
}