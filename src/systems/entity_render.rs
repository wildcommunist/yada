use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut batch_draw = DrawBatch::new();
    batch_draw.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);


    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            batch_draw.set(
                *pos - offset,
                render.color,
                render.glyph,
            );
        });

    batch_draw.submit(5000).expect("Failed to draw entities batch");
}