use crate::prelude::*;

const NUM_OF_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mapbuild = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mapbuild.fill(TileType::Wall);
        mapbuild.build_random_rooms(rng);
        mapbuild.build_corridors(rng);
        mapbuild.player_start = mapbuild.rooms[0].center();
        mapbuild
    }

    fn fill(&mut self, tile: TileType) {
        // modify each item in the vector
        // no auto-deref on 't'
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_OF_ROOMS {
            // create a room using Rect from bracket-lib
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;

            // use self.rooms.iter() instead of self.rooms as self.rooms consumes
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn make_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn make_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        // sort rooms so that corridors aren't overly long
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // connect rooms together
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                // Make horizontal then vertical corridor starting from previous room
                self.make_horizontal_tunnel(prev.x, new.x, prev.y);
                self.make_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                // Make vertical then horizontal corridor starting from previous room
                self.make_vertical_tunnel(prev.y, new.y, prev.x);
                self.make_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
