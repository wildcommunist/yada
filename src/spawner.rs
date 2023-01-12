use crate::prelude::*;

pub fn spawn_player(
    ecs: &mut World,
    position: Point,
) {
    ecs.push(
        (
            Player {},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    ecs.push(
        (
            Enemy {},
            Collider{},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rng.range(0, 4) {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g')
                },
            }
        )
    );
}

pub fn spawn_resource(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    ecs.push(
        (
            CollectableResource {},
            Collider{},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rng.range(0, 4) {
                    0 => to_cp437('K'),
                    1 => to_cp437('L'),
                    2 => to_cp437('M'),
                    _ => to_cp437('N')
                },
            }
        )
    );
}