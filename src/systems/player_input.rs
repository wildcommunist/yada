use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Resource)]
#[write_component(HealthPool)]
#[read_component(Collider)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    let (player_entity, player_position) = players
        .iter(ecs)
        .map(|(entity, pos)| Some((*entity, *pos))).next()
        .unwrap().unwrap();

    if let Some(key) = key {
        let delta = match key {

            // "action" buttons
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),

            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::G => {
                let mut items_query = <(Entity, &Item, &Point)>::query();
                items_query.iter(ecs)
                    .filter(|(_, _, &item_pos)| item_pos == player_position)
                    .for_each(|(item_entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*item_entity);
                        commands.add_component(*item_entity, Carried(player_entity));

                        let item_ref = ecs.entry_ref(*item_entity);
                        // Is this a weapon we picked up
                        let mut is_weapon = false;
                        if let Ok(item) = item_ref {
                            if item.get_component::<Weapon>().is_ok() {
                                is_weapon = true;
                            }
                        }

                        if is_weapon {
                            // we only need to drop the other weapon if we picking up another
                            if let Ok(ent_ref) = ecs.entry_ref(player_entity) {
                                <(Entity, &Carried, &Weapon)>::query().iter(ecs)
                                    .filter(|(_, c, _)| c.0 == player_entity)
                                    .for_each(|(ent, _, _)| {
                                        commands.add_component(*ent, Point::new(player_position.x, player_position.y));
                                        commands.remove_component::<Carried>(*ent);
                                    });
                            }
                        }
                    });
                Point::zero()
            }
            _ => Point::zero()
        };
        let player_destination = player_position + delta;

        let mut is_idle = true;

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
                        commands
                            .push(((), WantsToGather {
                                source: player_entity,
                                target: *entity,
                            }));
                    };

                    if ecs.entry_ref(*entity)
                        .unwrap().get_component::<Enemy>().is_ok() {
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
            /*
            if let Ok(health_component) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>() {
                health_component.current = i32::min(health_component.max, health_component.current + 1);
            }
            */
        }
        *state = TurnState::PlayerTurn;
    }
}

fn use_item(
    slot: usize,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) -> Point {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(a, _b)| *a)
        .next()
        .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .enumerate()
        .filter(|(count, (_, _, _))| *count == slot)// Get the item in slot
        .map(|(_, (ent, _, _))| ent)
        .next();

    if let Some(item_entity) = item_entity {
        commands
            .push(((), ActivateItem { user: player, item: *item_entity }));
    }

    Point::zero()
}