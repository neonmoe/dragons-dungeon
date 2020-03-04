/// Prototypes of different kinds of entities.
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
    visible: true,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health {
        current: 12,
        max: 12,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(4)),
    inventory: Some(Inventory::new()),
    ai: None,
    drop: None,
};

pub const PROTO_WALL: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::WALL),
    visible: true,
    animation: Animation::default(),
    denies_movement: true,
    health: None,
    status_effects: Some(Vec::new()),
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_SKELETON: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::SKELETON),
    visible: true,
    animation: Animation::default(),
    denies_movement: true,
    health: Some(Health { current: 8, max: 8 }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(4)),
    inventory: None,
    ai: Some(Ai::Skeleton(SkeletonAi::new())),
    drop: None,
};

pub const PROTO_COBWEB: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::COBWEB),
    visible: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_ZOMBIE: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::ZOMBIE),
    visible: true,
    animation: Animation::default(),
    denies_movement: false,
    health: Some(Health {
        current: 12,
        max: 12,
    }),
    status_effects: Some(Vec::new()),
    damage: Some(Damage(3)),
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_DRAGON: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::DRAGON),
    visible: true,
    animation: Animation::default(),
    denies_movement: false,
    health: Some(Health {
        current: 24,
        max: 24,
    }),
    status_effects: Some(Vec::new()),
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_APPLE: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::APPLE),
    visible: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_DOOR: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::DOOR),
    visible: true,
    animation: Animation::default(),
    denies_movement: true,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

const PROTO_ITEM: Entity = Entity {
    position: Position { x: 0, y: 0 },
    sprite: Sprite(sprites::ITEM_SWORD),
    visible: true,
    animation: Animation::default(),
    denies_movement: false,
    health: None,
    status_effects: None,
    damage: None,
    inventory: None,
    ai: None,
    drop: None,
};

pub const PROTO_ITEMS: [Entity; 7] = [
    Entity {
        sprite: Sprite(sprites::ITEM_SWORD),
        drop: Some(Item::Sword),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_SCYTHE),
        drop: Some(Item::Scythe),
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
        sprite: Sprite(sprites::ITEM_SHIELD),
        drop: Some(Item::Shield),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_VAMPIRE_TEETH),
        drop: Some(Item::VampireTeeth),
        ..PROTO_ITEM
    },
    Entity {
        sprite: Sprite(sprites::ITEM_STOPWATCH),
        drop: Some(Item::Stopwatch),
        ..PROTO_ITEM
    },
];
