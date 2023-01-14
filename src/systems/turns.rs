use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();

    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::WorldTurn,
        TurnState::WorldTurn => TurnState::AwaitingInput,
        _ => current_state
    };
    if player_hp.iter(ecs).next().unwrap().current < 1 {
        new_state = TurnState::GameOver;
    }

    *turn_state = new_state;
}