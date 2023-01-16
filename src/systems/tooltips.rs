use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(NameLabel)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(ItemRarity)]
pub fn tooltip(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    let mut positions = <(Entity, &Point, &NameLabel)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, p, _)| { **p == map_pos && player_fov.visible_tiles.contains(p) })
        .for_each(|(e, _, name)| {
            let screen_pos = Point::new((*mouse_pos * 4).x, (*mouse_pos * 4).y - 1);

            let color = match ecs.entry_ref(*e)
                .unwrap().get_component::<ItemRarity>() {
                Ok(r) => *r,
                Err(_) => ItemRarity::Unknown
            };

            let display = if let Ok(health) = ecs.entry_ref(*e)
                .unwrap().get_component::<Health>() {
                format!("{} ({}/{})", name.0, health.current, health.max)
            } else {
                name.0.clone()
            };
            draw_batch.print_color(screen_pos, &display, ColorPair::from(color));
        });

    draw_batch.submit(10100).expect("Tooltip batch error");
}