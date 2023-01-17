mod template;

use crate::prelude::*;
use crate::spawner::template::Templates;

pub fn spawn_player(
    ecs: &mut World,
    position: Point,
) {
    let player = ecs.push(
        (
            Player { map_level: 0 },
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health { current: 75, max: 150 },
            XP { current: 0, max: 100 },
        )
    );

    if let Some(mut e) = ecs.entry(player) {
        e.add_component(FieldOfView::new(8));
    }
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_item(
    ecs: &mut World,
    position: Point,
) {
    ecs.push(
        (
            Item, AmuletOfYala, position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|'),
            },
            NameLabel("Amulet of Yala".to_string())
        )
    );
}