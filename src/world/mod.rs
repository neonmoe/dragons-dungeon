mod entity;

use crate::sprites;
use entity::{Entity, Position, Sprite};

use fae::{Font, GraphicsContext, Spritesheet};

/// Represents an iterator over all entities except for one. Used when
/// running updates for a that one entity, if it needs to interact
/// with others.
///
/// Created internally by World::split_entities.
pub type EntityIter<'a> =
    std::iter::Chain<std::slice::Iter<'a, Entity>, std::slice::Iter<'a, Entity>>;

#[derive(Debug, Clone)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
}

#[derive(Debug, Clone)]
pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: vec![
                Entity {
                    position: Position { x: 0, y: 0 },
                    sprite: Sprite {
                        data: sprites::PLAYER,
                    },
                    denies_movement: true,
                },
                Entity {
                    position: Position { x: 1, y: 2 },
                    sprite: Sprite {
                        data: sprites::WALL,
                    },
                    denies_movement: true,
                },
            ],
        }
    }

    pub fn update(&mut self, action: PlayerAction) {
        let (player, others) = self.split_entities(0);
        match action {
            PlayerAction::MoveUp => move_entity(player, others, 0, -1),
            PlayerAction::MoveDown => move_entity(player, others, 0, 1),
            PlayerAction::MoveRight => move_entity(player, others, 1, 0),
            PlayerAction::MoveLeft => move_entity(player, others, -1, 0),
        };
    }

    pub fn animate(&mut self, _delta_seconds: f32) {
        // TODO: Animations for entities
    }

    pub fn render(&self, ctx: &mut GraphicsContext, _font: &Font, tileset: &Spritesheet) {
        let tile_size = 32;
        for (position, sprite) in self.entities.iter().map(|e| (&e.position, &e.sprite)) {
            tileset
                .draw(ctx)
                .coordinates((
                    position.x * tile_size,
                    position.y * tile_size,
                    tile_size,
                    tile_size,
                ))
                .texture_coordinates(sprite.data[0])
                .z(0.0)
                .finish();
        }
    }

    fn split_entities(&mut self, separated_index: usize) -> (&mut Entity, EntityIter) {
        let (head, tail) = self.entities.split_at_mut(separated_index);
        let (separated, tail) = tail.split_at_mut(1);
        (&mut separated[0], head.iter().chain(tail.iter()))
    }
}

fn move_entity(entity: &mut Entity, others: EntityIter, xd: i32, yd: i32) -> bool {
    let (new_x, new_y) = (entity.position.x + xd, entity.position.y + yd);
    for other in others.filter(|e| e.denies_movement) {
        if new_x == other.position.x && new_y == other.position.y {
            return false;
        }
    }
    entity.position.x = new_x;
    entity.position.y = new_y;
    true
}
