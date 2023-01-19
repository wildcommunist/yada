use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Collider)]
#[read_component(Point)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.point) {
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(
                    want_move.entity, fov.clone_dirty(),
                );

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.point);
                    fov.visible_tiles.iter().for_each(|visible_tile_pos| {
                        map.revealed_tiles[Map::map_index(visible_tile_pos.x, visible_tile_pos.y)] = true;
                    });
                }
            }
        }

        // Check that no1 has moved there already
        let mut positions = <(Entity, &Point)>::query().filter(component::<Collider>() | component::<Player>());

        let mut tile_taken = false;

        if positions.iter(ecs)
            .filter(|(_, pos)| **pos == want_move.point).count() > 0 {
            // there is somebody there already, touch tits
            println!("Tile {} {} is already taken", want_move.point.x, want_move.point.y);
            tile_taken = true;
        }

        if !tile_taken { // Still doesnt work.
            // Update the Point component of the entity (effectively "move")
            commands.add_component(
                want_move.entity,
                want_move.point,
            ); // This will replace the `Point` component on the `Entity` if it exists. The only point component we have on entities are their positions
        }
    }
    commands.remove(*entity);
}
