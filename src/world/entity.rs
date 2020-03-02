use crate::sprites::SpriteData;
use crate::world::ai::Ai;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub sprite: Sprite,
    pub animation: Animation,
    pub denies_movement: bool,
    pub health: Option<Health>,
    pub damage: Option<Damage>,
    pub inventory: Option<Inventory>,
    pub ai: Option<Ai>,
    pub drop: Option<Item>,
}

impl Entity {
    pub fn clone_at(&self, x: i32, y: i32) -> Entity {
        let mut new_entity = self.clone();
        new_entity.position = Position { x, y };
        new_entity
    }

    pub fn is_alive(&self) -> bool {
        if let Some(health) = &self.health {
            health.current > 0
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Sprite(pub SpriteData);

#[derive(Debug, Clone)]
pub struct Animation {
    pub x: AnimationState<f32>,
    pub y: AnimationState<f32>,
    pub opacity: AnimationState<f32>,
    pub rotation: AnimationState<f32>,
}

#[derive(Debug, Clone)]
pub struct AnimationState<T> {
    pub current: T,
    pub from: T,
    pub to: T,
}

#[derive(Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone)]
pub struct Damage(pub i32);

#[derive(Debug, Clone)]
pub struct Inventory {
    pub item_left: Option<Item>,
    pub item_right: Option<Item>,
}

impl Inventory {
    pub fn has_item(&self, item: Item) -> bool {
        self.item_left.iter().any(|i| *i == item) || self.item_right.iter().any(|i| *i == item)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// TODO: Add all the items into the game
#[allow(dead_code)]
pub enum Item {
    Sword,
    Scythe,
    Hammer,
    Dagger,
    Shield,
    VampireTeeth,
    Stopwatch,
}
