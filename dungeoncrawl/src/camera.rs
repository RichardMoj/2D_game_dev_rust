use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    // Center the camera on the player
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: self.x - DISPLAY_WIDTH / 2,
            right_x: self.x + DISPLAY_WIDTH / 2,
            top_y: self.y - DISPLAY_HEIGHT / 2,
            bottom_y: self.y + DISPLAY_HEIGHT / 2,
        }
    }

    // Update the camera
    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = self.x - DISPLAY_WIDTH / 2;
        self.right_x = self.x + DISPLAY_WIDTH / 2;
        self.top_y = self.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = self.y + DISPLAY_HEIGHT / 2;
    }
}
