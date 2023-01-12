use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut batch_draw = DrawBatch::new();
    batch_draw.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            batch_draw.set(
                *pos - offset,
                render.color,
                render.glyph,
            );
        });

    batch_draw.submit(5000).expect("Failed to draw entities batch");
}