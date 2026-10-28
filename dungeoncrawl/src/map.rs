use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        ctx.set_active_console(0);
        for y in cam.top_y..cam.bottom_y {
            for x in cam.left_x..cam.right_x {
                if self.bounds_check(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(x - cam.left_x, y - cam.top_y, YELLOW, BLACK, to_cp437('.'))
                        }
                        TileType::Wall => {
                            ctx.set(x - cam.left_x, y - cam.top_y, GREEN, BLACK, to_cp437('#'))
                        }
                    }
                }
            }
        }
    }

    // returns true if the point is inside the map bounds
    pub fn bounds_check(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn enter_tile(&self, point: Point) -> bool {
        let idx = map_idx(point.x, point.y);
        self.bounds_check(point) && self.tiles[idx] == TileType::Floor
    }

    // returns an error if the point is outside the map bounds
    // or the converted index of an x,y pair
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.bounds_check(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
