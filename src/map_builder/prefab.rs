use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#RRRR#---
---#----#---
-###----#---
--M-----#---
-###----#---
---#M---#---
---#----###-
---#----M---
---########-
", 12, 11);

pub fn apply_prefab(
    mb: &mut MapBuilder,
    rng: &mut RandomNumberGenerator,
) {
    let mut placement = None;
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH, MAP_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let prefab_rect = Rect::with_size(
            rng.range(0, MAP_WIDTH - FORTRESS.1),
            rng.range(0, MAP_WIDTH - FORTRESS.2),
            FORTRESS.1, FORTRESS.2,
        );

        let mut can_place = false;
        prefab_rect.for_each(|prefab_tile| {
            let idx = mb.map.point2d_to_index(prefab_tile);
            let distance = dijkstra_map.map[idx];
            if distance < 2000. && distance > 20.0 && mb.amulet_start != prefab_tile {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(prefab_rect.x1, prefab_rect.y1));
            let points = prefab_rect.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
            mb.resource_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    println!("Attempts: {}", attempts);

    if let Some(prefab) = placement {
        println!("Prefab location: X:{} y:{}", prefab.x, prefab.y);
        let string_vec: Vec<char> = FORTRESS.0
            .chars()
            .filter(|c| *c != '\r' && *c != '\n')
            .collect();
        let mut i = 0;
        for ty in prefab.y..prefab.y + FORTRESS.2 {
            for tx in prefab.x..prefab.x + FORTRESS.1 {
                let idx = Map::map_index(tx, ty);
                let c = string_vec[i];

                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    'R' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.resource_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with character `{}`", c),
                }
                i += 1;
            }
        }
    }
}