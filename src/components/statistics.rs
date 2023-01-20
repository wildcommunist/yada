#[derive(Clone, Debug, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Skills(pub Vec<Skill>);


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HealthPool {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ManaPool {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPPool {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weapon;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Damage(pub u32);

pub trait Modifier {
    fn apply(&self) -> i32;
}