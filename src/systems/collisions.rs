use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Point)]
pub fn collisions(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|p_pos| player_pos = *p_pos); // Get player position

    let mut collidables = <(Entity, &Point)>::query()
        .filter(!component::<Player>() & component::<Collider>());

    collidables
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}