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
    let mut render_query = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut batch_draw = DrawBatch::new();
    batch_draw.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);


    render_query
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            batch_draw.set(
                *pos - offset,
                render.color,
                render.glyph,
            );
        });

    // We query specifically for the player so we render them "last" so they look on top of things
    let mut player_render_query = <(&Point, &Render, &Player)>::query();
    let (position, player) = player_render_query.iter(ecs)
        .map(|(pos, render, _)| (pos, render))
        .next().unwrap();
    batch_draw.set(
        *position - offset,
        player.color,
        player.glyph,
    );

    batch_draw.submit(5000).expect("Failed to draw entities batch");
}