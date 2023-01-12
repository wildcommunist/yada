use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Wanderer)]
pub fn random_move(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut movers = <&mut Point>::query()
        .filter(component::<Wanderer>());

    movers
        .iter_mut(ecs)
        .for_each(|p| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 5) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                3 => Point::new(0, 1),
                _ => Point::new(0, 0)
            } + *p;

            if map.can_enter_tile(destination) {
                *p = destination;
            }
        });
}