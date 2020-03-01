use crate::sprites::SpriteData;
use crate::world::ai::Ai;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub sprite: Sprite,
    pub animation: Animation,
    pub denies_movement: bool,
    pub health: Option<Health>,
    pub ai: Option<Ai>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub data: SpriteData,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
