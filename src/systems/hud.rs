use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(XP)]
pub fn hud(
    ecs: &SubWorld
) {
    let mut health_query = <&Health>::query()
        .filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .next()
        .unwrap();
    let mut xp_query = <&XP>::query()
        .filter(component::<Player>());
    let player_xp = xp_query
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Draw the health bar
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" HP: {} / {} ", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );
    // Draw the XP bar
    draw_batch.bar_horizontal(
        Point::new(0, 1),
        SCREEN_WIDTH * 2,
        player_xp.current,
        player_xp.max,
        ColorPair::new(GREEN, BLACK),
    );
    draw_batch.print_color_centered(
        1,
        format!(" XP: {} / {} ", player_xp.current, player_xp.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(10000).expect("Failed to batch draw the HUD");
}