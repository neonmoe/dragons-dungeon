//! Prototypes of different kinds of entities.
use super::ai::*;
use super::entity::*;
use crate::sprites;

impl Animation {
    pub const fn default() -> Animation {
        Animation {
            x: AnimationState {
                current: 0.0,
                from: 0.0,
                to: 0.0,
            },
            y: AnimationState {
                current: 0.0,
                from: 0.0,
                to: 0.0,
            },
            opacity: AnimationState {
                current: 1.0,
                from: 1.0,
                to: 1.0,
            },
            rotation: AnimationState {
                current: 0.0,
                from: 0.0,
                to: 0.0,
            },
        }
    }
}

pub const PROTO_PLAYER: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::PLAYER),
    visibility_affected: false,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health {
        current: 12,
        max: 12,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(2)),
    inventory: Some(Inventory::new()),
    ai: None,
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_WALL: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::WALL),
    visibility_affected: false,
    animation: Animation::default(),
    denies_movement: true,
    health: None,
    status_effects: Some(Vec::new()),
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_SKELETON: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::SKELETON),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health {
        current: 10,
        max: 10,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(2)),
    inventory: None,
    ai: Some(Ai::Skeleton),
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

#[allow(dead_code)]
pub const PROTO_COBWEB: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::COBWEB),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_ZOMBIE: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::ZOMBIE),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health {
        current: 12,
        max: 12,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(2)),
    inventory: None,
    ai: Some(Ai::Zombie),
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_DRAGON: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::DRAGON),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health {
        current: 4 * 9,
        max: 4 * 9,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(5)),
    inventory: None,
    ai: Some(Ai::Dragon),
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: true,
};

pub const PROTO_FLAME: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::FLAME),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: Some(Damage(3)),
    inventory: None,
    ai: Some(Ai::Flame),
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_DOOR: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::DOOR),
    visibility_affected: false,
    animation: Animation::default(),
    denies_movement: true,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
    marked_for_death: false,
    door: true,
    next_level: false,
    dragon: false,
};

pub const PROTO_NEXT_LEVEL: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::NEXT_LEVEL),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: true,
    dragon: false,
};

const PROTO_ITEM: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::ITEM_SWORD),
    visibility_affected: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
    marked_for_death: false,
    door: false,
    next_level: false,
    dragon: false,
};

pub const PROTO_APPLE: Entity = Entity {
    sprite: Sprite(sprites::ITEM_APPLE),
    drop: Some(Item::Apple),
    ..PROTO_ITEM
};

pub const PROTO_ITEMS: [Entity; 7] = [
    Entity {
        sprite: Sprite(sprites::ITEM_SWORD),
        drop: Some(Item::Sword),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_SHIELD),
        drop: Some(Item::Shield),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_HAMMER),
        drop: Some(Item::Hammer),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_DAGGER),
        drop: Some(Item::Dagger),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_VAMPIRE_TEETH),
        drop: Some(Item::VampireTeeth),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_SCYTHE),
        drop: Some(Item::Scythe),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_STOPWATCH),
        drop: Some(Item::Stopwatch(false)),
        ..PROTO_ITEM
    },
];
