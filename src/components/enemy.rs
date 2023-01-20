use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}