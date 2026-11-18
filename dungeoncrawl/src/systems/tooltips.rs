use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &mut SubWorld, #[resource] camera: &Camera, #[resource] mouse_pos: &Point) {
    let mut positions = <(&Point, &Name, Entity)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);

    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(p, _, _)| **p == map_pos)
        .for_each(|(_, n, e)| {
            // multiply by 4 because tooltips layer is 4 times larger
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(health) = ecs.entry_ref(*e).unwrap().get_component::<Health>() {
                format!("{} : {} hp", &n.0, health.current)
            } else {
                n.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
