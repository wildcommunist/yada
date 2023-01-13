use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chase(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).next().unwrap().0;
    let player_idx = Map::map_index(player_position.x, player_position.y);

    let search_targets = vec![player_idx];

    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &search_targets,
        map,
        1024.0,
    );

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = Map::map_index(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(
            &dijkstra_map, idx, map,
        ) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_position);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_position
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(victim, _, _)| {
                    if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
                        commands
                            .push(((), WantsToAttack {
                                attacker: *entity,
                                target: *victim,
                            }));
                    }
                    attacked = true;
                });
            if !attacked {
                commands
                    .push(((), WantsToMove { entity: *entity, point: destination }));
            }
        }
    })
}