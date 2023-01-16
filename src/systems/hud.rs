use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(XP)]
#[read_component(Item)]
#[read_component(ItemRarity)]
#[read_component(Carried)]
#[read_component(NameLabel)]
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
    let (player_entity, player) = <(Entity, &Player)>::query()
        .iter(ecs)
        .next()
        .map(|(e, player)| (*e, player))
        .unwrap();
    let player_xp = xp_query
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    draw_batch.print_color_right(
        Point::new(
            SCREEN_WIDTH * 2,
            3,
        ), format!("Dungeon level: {}", player.map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 5;
    <(&Item, &NameLabel, &ItemRarity, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, _, carried)| carried.0 == player_entity)
        .for_each(|(_, name, rarity, _)| {
            // "draw" the inventory
            draw_batch.print_color(Point::new(3, y), format!("{}. {}", y - 4, &name.0), ColorPair::from(*rarity));
            y += 1;
        });

    if y > 5 {
        draw_batch.print_color(Point::new(2, 3), "Inventory:", ColorPair::new(YELLOW, BLACK));
    }

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