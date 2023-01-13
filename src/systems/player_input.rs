use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Resource)]
#[write_component(Health)]
#[read_component(Collider)]
#[read_component(Enemy)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::zero()
        };

        let mut is_idle = true;

        let mut players = <(Entity, &Point)>::query()
            .filter(component::<Player>());
        let (player_entity, player_destination) = players
            .iter(ecs)
            .map(|(entity, pos)| Some((*entity, *pos + delta))).next()
            .unwrap().unwrap();

        if delta.x != 0 || delta.y != 0 {
            is_idle = false;
            let mut colliders = <(Entity, &Point)>::query()
                .filter(component::<Collider>());
            let mut hit_something = false;

            colliders
                .iter(ecs)
                .filter(|(_, pos)| **pos == player_destination) // we only want entities that are in the cell we want to move
                .for_each(|(entity, _)| {
                    // we bumped into something! What is it? Do we want to attack or gather?
                    if let Ok(res) = ecs.entry_ref(*entity)
                        .unwrap().get_component::<Resource>() {
                        println!("We got a resource: {:?}", res.resource);
                        println!("We got a player: {:?}", player_entity);
                        commands
                            .push(((), WantsToGather {
                                source: player_entity,
                                target: *entity,
                            }));
                        println!("Pushed resource {:?}", res.resource);
                    };

                    if ecs.entry_ref(*entity)
                        .unwrap().get_component::<Enemy>().is_ok() {
                        println!("We got an enemy!");
                        commands
                            .push(((), WantsToAttack {
                                attacker: player_entity,
                                target: *entity,
                            }));
                    };

                    hit_something = true;
                });

            if !hit_something {
                // empty cell, show our intent to move there
                commands
                    .push(((), WantsToMove {
                        entity: player_entity,
                        point: player_destination,
                    }));
            }
        }
        if is_idle {
            if let Ok(health_component) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>() {
                health_component.current = i32::min(health_component.max, health_component.current + 1);
            }
        }
        *state = TurnState::PlayerTurn;
    }
}