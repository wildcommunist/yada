use crate::prelude::*;

#[system]
#[read_component(HealthPool)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
) {
    let mut player_hp = <(&HealthPool, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_default_position = Point::new(-1, -1);
    let amulet_position = amulet
        .iter(ecs)
        .next()
        .unwrap_or(&amulet_default_position);

    let current_state = *turn_state;

    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::WorldTurn,
        TurnState::WorldTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_position {
            new_state = TurnState::Victory;
        }

        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Portal {
            // we stumbled upon the exit
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}