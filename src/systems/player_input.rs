use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
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

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query()
                .filter(component::<Player>());

            players.iter_mut(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands
                    .push(((), WantsToMove { entity: *entity, point: destination }));
            })
        }
        *state = TurnState::PlayerTurn;
    }
}