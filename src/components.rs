use std::fmt::{Display, Formatter};
use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectableResource {}

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
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XP {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Mithril,
    Adamatite,
    Crokite,
    Silver,
    Gold,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Resource {
    pub resource: ResourceType,
    pub amount: u8,
}

#[derive(Clone, PartialEq)]
pub struct NameLabel(pub String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToPerformAction {
    pub issuer: Entity,
    pub action: Action,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Attack,
    Gather,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToGather {
    pub source: Entity,
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;