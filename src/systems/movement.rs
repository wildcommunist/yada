use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
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

        commands.add_component(
            want_move.entity,
            want_move.point,
        ); // This will replace the `Point` component on the `Entity` if it exists. The only point component we have on entities are their positions
    }
    commands.remove(*entity);
}
