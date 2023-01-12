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

