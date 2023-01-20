use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collider {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Wanderer {} //TODO: Limit wandering by setting home point and checking distance from it

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub point: Point,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;