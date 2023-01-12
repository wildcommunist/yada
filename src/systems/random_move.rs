use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Wanderer)]
pub fn random_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point)>::query()
        .filter(component::<Wanderer>());

    movers
        .iter_mut(ecs)
        .for_each(|(e, p)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 5) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                3 => Point::new(0, 1),
                _ => Point::new(0, 0)
            } + *p;

            commands
                .push(((), WantsToMove { entity: *e, point: destination }));
        });
}