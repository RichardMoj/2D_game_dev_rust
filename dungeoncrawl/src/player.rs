use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        ctx.set_active_console(1);

        // Subtract from cam to convert from world-space to screen-space
        ctx.set(
            self.position.x - cam.left_x,
            self.position.y - cam.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, map: &Map, ctx: &mut BTerm, cam: &mut Camera) {
        if let Some(key) = ctx.key {
            let change = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::new(0, 0),
            };

            let new_position = self.position + change;
            if map.enter_tile(new_position) {
                self.position = new_position;
                cam.on_player_move(new_position);
            }
        }
    }
}
