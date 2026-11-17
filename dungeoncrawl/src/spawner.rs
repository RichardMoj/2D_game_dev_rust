use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pnt: Point) {
    ecs.push((
        Player,
        pnt,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            maximum: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MovingRandomly {},
        Health {
            current: hp,
            maximum: hp,
        },
        name,
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, String::from("Goblin"), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, String::from("Orc"), to_cp437('o'))
}
