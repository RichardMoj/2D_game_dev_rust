use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pnt: Point) {
    ecs.push((
        Player,
        pnt,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
