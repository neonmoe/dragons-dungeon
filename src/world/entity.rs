use crate::sprites::SpriteData;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub sprite: Sprite,
    pub denies_movement: bool,
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
