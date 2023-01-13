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
            },
            Health { current: 1, max: 150 },
            XP { current: 99, max: 100 },
        )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        0 => goblin(),
        _ => orc()
    };

    ecs.push(
        (
            Enemy {},
            Collider {},
            Wanderer {},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            Health { current: hp, max: hp },
            NameLabel(name)
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (10, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (15, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_resource(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    ecs.push(
        (
            CollectableResource {},
            Collider {},
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