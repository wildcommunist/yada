use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(NameLabel)]
pub fn tooltip(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    let mut positions = <(Entity, &Point, &NameLabel)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, p, _)| { **p == map_pos })
        .for_each(|(e, _, name)| {
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(health) = ecs.entry_ref(*e)
                .unwrap().get_component::<Health>() {
                format!("{} ({}/{})", name.0, health.current, health.max)
            } else {
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Tooltip batch error");
}