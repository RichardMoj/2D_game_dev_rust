mod entity_reader;
mod map_render;
mod player_input;
use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_reader::entity_reader_system())
        .build()
}
