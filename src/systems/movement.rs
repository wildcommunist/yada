use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.point) {
        // we can move to the destination we need to go
        //TODO: Checks that we actually can move there
        commands.add_component(want_move.entity, want_move.point); // This will replace the `Point` component on the `Entity` if it exists. The only point component we have on entities are their positions

        if ecs.entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.point);
        }
    }
    commands.remove(*entity);
}
