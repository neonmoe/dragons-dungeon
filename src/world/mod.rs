mod ai;
mod entity;

use crate::sprites;
use ai::{Ai, SkeletonAi};
use entity::{Entity, Health, Position, Sprite};

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
    Wait,
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
                    health: Some(Health {
                        current: 16,
                        max: 16,
                    }),
                    ai: None,
                },
                Entity {
                    position: Position { x: 1, y: 2 },
                    sprite: Sprite {
                        data: sprites::WALL,
                    },
                    denies_movement: true,
                    health: None,
                    ai: None,
                },
                Entity {
                    position: Position { x: 5, y: 5 },
                    sprite: Sprite {
                        data: sprites::SKELETON,
                    },
                    denies_movement: true,
                    health: Some(Health { current: 8, max: 8 }),
                    ai: Some(Ai::Skeleton(SkeletonAi::new())),
                },
            ],
        }
    }

    pub fn update(&mut self, action: PlayerAction) {
        // Update player
        self.update_player(action);

        // Update the rest of the entities, in order
        for i in 1..self.entities.len() {
            let (entity, others) = self.split_entities(i);
            if let Some(ai) = &mut entity.ai {
                match ai {
                    Ai::Skeleton(ai) => ai.update(&mut entity.position, others),
                }
            }
        }
    }

    fn update_player(&mut self, action: PlayerAction) {
        let (player, others) = self.split_entities(0);
        let mut move_direction = None;
        match action {
            PlayerAction::MoveUp => move_direction = Some((0, -1)),
            PlayerAction::MoveDown => move_direction = Some((0, 1)),
            PlayerAction::MoveRight => move_direction = Some((1, 0)),
            PlayerAction::MoveLeft => move_direction = Some((-1, 0)),
            PlayerAction::Wait => {}
        };
        if let Some((xd, yd)) = move_direction {
            move_entity(&mut player.position, others, xd, yd);
        }
    }

    pub fn animate(&mut self, _delta_seconds: f32) {
        // TODO: Animations for entities
    }

    pub fn render(&self, ctx: &mut GraphicsContext, _font: &Font, tileset: &Spritesheet) {
        let tile_size = 32;
        let drawable_width = (ctx.width - crate::ui::UI_AREA_WIDTH) as i32;
        let drawable_height = ctx.height as i32;
        let offset = (
            drawable_width / 2 - (self.entities[0].position.x * 2 + 1) * tile_size / 2,
            drawable_height as i32 / 2 - (self.entities[0].position.y * 2 + 1) * tile_size / 2,
        );
        for (position, sprite) in self.entities.iter().map(|e| (&e.position, &e.sprite)) {
            tileset
                .draw(ctx)
                .coordinates((
                    position.x * tile_size + offset.0,
                    position.y * tile_size + offset.1,
                    tile_size,
                    tile_size,
                ))
                .texture_coordinates(sprite.data[0])
                .z(0.0)
                .finish();
        }

        for (position, health) in self
            .entities
            .iter()
            .filter_map(|e| e.health.as_ref().map(|health| (&e.position, health)))
        {
            let pos = (
                position.x * tile_size + offset.0,
                position.y * tile_size + offset.1,
            );
            let dark = (0.2, 0.5, 0.8, 0.8);
            let light = (0.9, 0.1, 0.0, 1.0);
            draw_hearts(ctx, tileset, pos, tile_size, dark, health.max);
            draw_hearts(ctx, tileset, pos, tile_size, light, health.current);
        }
    }

    fn split_entities(&mut self, separated_index: usize) -> (&mut Entity, EntityIter) {
        let (head, tail) = self.entities.split_at_mut(separated_index);
        let (separated, tail) = tail.split_at_mut(1);
        (&mut separated[0], head.iter().chain(tail.iter()))
    }
}

fn move_entity(position: &mut Position, others: EntityIter, xd: i32, yd: i32) -> bool {
    let (new_x, new_y) = (position.x + xd, position.y + yd);
    for other in others.filter(|e| e.denies_movement) {
        if new_x == other.position.x && new_y == other.position.y {
            return false;
        }
    }
    position.x = new_x;
    position.y = new_y;
    true
}

fn draw_hearts(
    ctx: &mut GraphicsContext,
    tileset: &Spritesheet,
    (x, y): (i32, i32),
    tile_size: i32,
    tint: (f32, f32, f32, f32),
    heart_quarters: i32,
) {
    let hearts_total = (heart_quarters as f32 / 4.0).ceil() as i32;
    let rows = (hearts_total as f32 / 3.0).ceil() as i32;
    for i in 0..hearts_total {
        let coords = (
            x + tile_size / 4 * (i % 3) + tile_size / 8,
            y - tile_size / 4 * rows + tile_size / 4 * (i / 3),
            tile_size / 4,
            tile_size / 4,
        );
        let quarters = (4 - (heart_quarters - i * 4)).max(0) as usize;
        tileset
            .draw(ctx)
            .coordinates(coords)
            .texture_coordinates(sprites::ICONS_HEART[quarters][0])
            .z(0.1)
            .color(tint)
            .finish();
    }
}
