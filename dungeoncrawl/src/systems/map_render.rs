use crate::prelude::*;

#[system]
#[read_access(Map)]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0); // The equivalent of set context
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.bounds_check(pt) {
                let newpoint = map_idx(x, y);
                let glyph = match map.tiles[newpoint] {
                    TileType::Wall => to_cp437('#'),
                    TileType::Floor => to_cp437('.'),
                };
            }
            draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
