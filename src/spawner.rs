use crate::prelude::*;

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

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        0 => goblin(),
        _ => orc()
    };

    let entity_id = ecs.push(
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
            NameLabel(name),
        )
    );


    if let Some(mut e) = ecs.entry(entity_id) {
        e.add_component(ChasingPlayer);
        e.add_component(FieldOfView::new(5));
        e.add_component(ItemRarity::Poor);
    }
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
    let (amount, name, glyph, res_type) = spawn_resource_node(rng);

    ecs.push(
        (
            CollectableResource {},
            Collider {},
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            Resource { resource: res_type, amount },
            NameLabel(name)
        )
    );
}

fn spawn_resource_node(rng: &mut RandomNumberGenerator) -> (u8, String, FontCharType, ResourceType) {
    let (name, glyph, resource_type) = match rng.range(0, 10) {
        0..=4 => {
            ("Coal".to_string(), to_cp437('K'), ResourceType::Coal)
        }
        5..=6 => {
            ("Mithril".to_string(), to_cp437('L'), ResourceType::Coal)
        }
        7..=8 => {
            ("Adamantium".to_string(), to_cp437('M'), ResourceType::Coal)
        }
        9 => {
            ("Rune".to_string(), to_cp437('N'), ResourceType::Coal)
        }
        _ => {
            ("Coal".to_string(), to_cp437('K'), ResourceType::Coal)
        }
    };

    (rng.range(0, 4), name, glyph, resource_type)
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

pub fn spawn_loot_item(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    position: Point,
) {
    let roll = rng.roll_dice(1, 6);

    match roll {
        1 => spawn_potion(ecs, position),
        2 => spawn_dungeon_map(ecs, position),
        3 => spawn_resource(ecs, rng, position),
        _ => spawn_monster(ecs, rng, position)
    }
}

fn spawn_potion(
    ecs: &mut World,
    position: Point,
) {
    ecs.push(
        (
            Item, position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('!'),
            },
            ItemRarity::Poor,
            ProvidesHealing { amount: 5 },
            NameLabel("Insignificant healing potion".to_string())
        )
    );
}

fn spawn_dungeon_map(
    ecs: &mut World,
    position: Point,
) {
    ecs.push(
        (
            Item, position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('{'),
            },
            ItemRarity::Rare,
            ProvidesDungeonMap,
            NameLabel("Map of the dungeon".to_string())
        )
    );
}