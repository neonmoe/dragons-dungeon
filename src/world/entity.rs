use crate::sprites::{self, SpriteData};
use crate::world::ai::Ai;
use std::iter::Chain;
use std::option::IterMut;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Position,
    pub sprite: Sprite,
    pub visibility_affected: bool,
    pub animation: Animation,
    pub denies_movement: bool,
    pub health: Option<Health>,
    pub status_effects: Option<Vec<StatusEffect>>,
    pub damage: Option<Damage>,
    pub inventory: Option<Inventory>,
    pub ai: Option<Ai>,
    pub drop: Option<Item>,
    pub marked_for_death: bool,
    pub door: bool,
    pub next_level: bool,
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
            // Garbage collected entities are, for sure, not
            // alive. Walls can be though.
            !self.marked_for_death
        }
    }

    pub fn visibility_affected(&self) -> bool {
        self.is_alive() && self.visibility_affected
    }

    pub fn can_act(&self) -> bool {
        let stunned = if let Some(status_effects) = &self.status_effects {
            status_effects.contains(&StatusEffect::Stun)
        } else {
            false
        };
        self.is_alive() && !stunned && !self.marked_for_death
    }

    pub fn tick_status_effects(&mut self) {
        if let Some(status_effects) = &mut self.status_effects {
            let mut stunned = false;
            for status_effect in status_effects.iter_mut() {
                match status_effect {
                    StatusEffect::Stun => {
                        stunned = true;
                    }
                    StatusEffect::StunImmunity => {}
                    StatusEffect::Poison { stacks, duration } => {
                        if let Some(health) = &mut self.health {
                            health.current = (health.current - *stacks).max(0);
                        }
                        *duration -= 1;
                    }
                }
            }
            status_effects.retain(|status_effect| match status_effect {
                StatusEffect::Stun => false,
                StatusEffect::StunImmunity => false,
                StatusEffect::Poison { duration, .. } => *duration > 0,
            });
            if stunned {
                status_effects.push(StatusEffect::StunImmunity);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum StatusEffect {
    Stun,
    StunImmunity,
    Poison { stacks: i32, duration: i32 },
}

#[derive(Debug, Clone)]
pub struct Damage(pub i32);

#[derive(Debug, Clone)]
pub struct Inventory {
    pub item_left: Option<Item>,
    pub item_right: Option<Item>,
    older_item: Option<ItemIndex>,
}

#[derive(Debug, Clone)]
enum ItemIndex {
    Left,
    Right,
}

impl Inventory {
    pub const fn new() -> Inventory {
        Inventory {
            item_left: None,
            item_right: None,
            older_item: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.item_left.is_none() && self.item_right.is_none()
    }

    pub fn has_item(&self, item: Item) -> bool {
        self.item_left.iter().any(|i| *i == item) || self.item_right.iter().any(|i| *i == item)
    }

    pub fn iter_mut(&mut self) -> Chain<IterMut<Item>, IterMut<Item>> {
        self.item_left.iter_mut().chain(self.item_right.iter_mut())
    }

    pub fn add_item(&mut self, item: Item) -> Option<Item> {
        if let Some(older_item) = &mut self.older_item {
            match older_item {
                ItemIndex::Left => {
                    self.older_item = Some(ItemIndex::Right);
                    let thrown_out = self.item_left;
                    self.item_left = Some(item);
                    thrown_out
                }
                ItemIndex::Right => {
                    self.older_item = Some(ItemIndex::Left);
                    let thrown_out = self.item_right;
                    self.item_right = Some(item);
                    thrown_out
                }
            }
        } else {
            self.older_item = Some(ItemIndex::Right);
            self.item_left = Some(item);
            None
        }
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
    Stopwatch(bool),
    Apple,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Sword => "Sword",
            Item::Scythe => "Scythe",
            Item::Hammer => "Hammer",
            Item::Dagger => "Dagger",
            Item::Shield => "Shield",
            Item::VampireTeeth => "Garlic",
            Item::Stopwatch(tick) => {
                if *tick {
                    "Stopwatch (tick)"
                } else {
                    "Stopwatch (tock)"
                }
            }
            Item::Apple => "Apple",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Item::Sword => "[describes a sword]",
            Item::Scythe => "[describes a scythe]",
            Item::Hammer => "[describes a hammer]",
            Item::Dagger => "[describes a dagger]",
            Item::Shield => "[describes a shield]",
            Item::VampireTeeth => "[describes a garlic]",
            Item::Stopwatch(tick) => "[describes a stopwatch]",
            Item::Apple => "[describes an apple]",
        }
    }

    pub fn sprite(&self) -> Sprite {
        match self {
            Item::Sword => Sprite(sprites::ITEM_SWORD),
            Item::Scythe => Sprite(sprites::ITEM_SCYTHE),
            Item::Hammer => Sprite(sprites::ITEM_HAMMER),
            Item::Dagger => Sprite(sprites::ITEM_DAGGER),
            Item::Shield => Sprite(sprites::ITEM_SHIELD),
            Item::VampireTeeth => Sprite(sprites::ITEM_VAMPIRE_TEETH),
            Item::Stopwatch(_) => Sprite(sprites::ITEM_STOPWATCH),
            Item::Apple => Sprite(sprites::ITEM_APPLE),
        }
    }
}
