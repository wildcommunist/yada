use crate::prelude::*;

#[system]
pub fn turn(
    #[resource] turn_state: &mut TurnState
) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::WorldTurn,
        TurnState::WorldTurn => TurnState::AwaitingInput,
    };
    *turn_state = new_state;
}