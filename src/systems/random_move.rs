use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Collider)]
#[read_component(Wanderer)]
#[read_component(Resource)]
#[read_component(HealthPool)]
#[read_component(Player)]
pub fn random_move(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point)>::query()
        .filter(component::<Wanderer>());
    let mut positions = <(Entity, &Point)>::query().filter(component::<Collider>() | component::<Player>());

    movers
        .iter(ecs)
        .for_each(|(current_entity, p)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 5) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                3 => Point::new(0, 1),
                _ => Point::new(0, 0)
            } + *p;

            // Check destination for potential targets, ignore everything else but the player
            let mut can_move = true;
            positions
                .iter(ecs)
                .filter(|(_, target_pos)| **target_pos == destination)// Grab all targets at destination
                .for_each(|(target, _target_position)| {
                    // we gotta make sure that if its an entity
                    // , we dont move there, if its a player, we attack
                    //TODO: Maybe monsters attack other factions?
                    if ecs.entry_ref(*target)
                        .unwrap().get_component::<Player>().is_ok() {
                        // this is a player
                        commands.push(((), WantsToAttack {
                            attacker: *current_entity,
                            target: *target,
                        }));
                        can_move = false;
                    }
                });
            if can_move {
                commands.push(((), WantsToMove {
                    entity: *current_entity,
                    point: destination,
                }));
            }
        });
}