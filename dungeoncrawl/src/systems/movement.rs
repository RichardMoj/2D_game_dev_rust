use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    if map.enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        // If the entity the system is working on is the player, then update the camera position to follow the player
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }

    // Remove messages otherwise processed messages will be processed again
    commands.remove(*entity);
}
