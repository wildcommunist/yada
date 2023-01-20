use crate::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectableResource {}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    CopperOre,
    IronOre,
    MithrilOre,
    Unknown,
}

impl From<&String> for ResourceType {
    fn from(value: &String) -> Self {
        match value.to_lowercase().as_str() {
            "copperore" => ResourceType::CopperOre,
            "ironore" => ResourceType::IronOre,
            "mithrilore" => ResourceType::MithrilOre,
            _ => ResourceType::Unknown
        }
    }
}

impl From<ResourceType> for String {
    fn from(value: ResourceType) -> Self {
        match value {
            ResourceType::CopperOre => String::from("Copper Ore"),
            ResourceType::IronOre => String::from("Iron Ore"),
            ResourceType::MithrilOre => String::from("Mithril Ore"),
            ResourceType::Unknown => String::from("Unknown Ore"),
        }
    }
}

impl From<ResourceType> for ItemRarity {
    fn from(value: ResourceType) -> Self {
        match value {
            ResourceType::CopperOre => ItemRarity::Common,
            ResourceType::IronOre => ItemRarity::Uncommon,
            ResourceType::MithrilOre => ItemRarity::Rare,
            ResourceType::Unknown => ItemRarity::Poor
        }
    }
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

impl From<Resource> for String {
    fn from(value: Resource) -> Self {
        match value.resource {
            ResourceType::CopperOre => String::from("Copper Ore"),
            ResourceType::IronOre => String::from("Iron Ore"),
            ResourceType::MithrilOre => String::from("Mithril Ore"),
            ResourceType::Unknown => String::from("Unknown Ore"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToGather {
    pub source: Entity,
    pub target: Entity,
}