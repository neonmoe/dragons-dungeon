mod ai;
mod entity;

use crate::sprites;
use ai::{Ai, SkeletonAi};
use entity::{Animation, Entity, Health, Position, Sprite};

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
    /// An ever-permanent entity collection, which should be
    /// initialized all at once, and never removed from.
    entities: Vec<Entity>,

    /// Every round we'll copy the whole state over to this Vec, just
    /// so animations can be done nicely. Sounds wasteful, probably
    /// is, but honestly, the whole game state is probably a few megs
    /// at most.
    previous_round_entities: Option<Vec<Entity>>,
    animation_timer: f32,
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
                    animation: Animation { x: 0.0, y: 0.0 },
                    denies_movement: true,
                    health: Some(Health {
                        current: 24,
                        max: 24,
                    }),
                    ai: None,
                },
                Entity {
                    position: Position { x: 1, y: 2 },
                    sprite: Sprite {
                        data: sprites::WALL,
                    },
                    animation: Animation { x: 0.0, y: 0.0 },
                    denies_movement: true,
                    health: None,
                    ai: None,
                },
                Entity {
                    position: Position { x: 5, y: 5 },
                    sprite: Sprite {
                        data: sprites::SKELETON,
                    },
                    animation: Animation { x: 0.0, y: 0.0 },
                    denies_movement: true,
                    health: Some(Health { current: 8, max: 8 }),
                    ai: Some(Ai::Skeleton(SkeletonAi::new())),
                },
            ],
            previous_round_entities: None,
            animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, action: PlayerAction) {
        // Huuuge clone, I know. See the docs for
        // `self.previous_round_entities`.
        self.previous_round_entities = Some(self.entities.clone());
        self.animation_timer = 0.0;

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

    pub fn animate(&mut self, delta_seconds: f32) {
        let previous_round_entities = match &self.previous_round_entities {
            Some(entities) => entities,
            None => return,
        };

        if self.animation_timer == 0.0 {
            // First frame of the current round, update everything
            // accordingly.
            for i in 0..self.entities.len() {
                let current = &mut self.entities[i];
                let previous = &previous_round_entities[i];
                let (xd, yd) = (
                    current.position.x - previous.position.x,
                    current.position.y - previous.position.y,
                );
                current.animation.x -= xd as f32;
                current.animation.y -= yd as f32;
            }
        } else {
            let clamped_lerp = |a: f32, b: f32, x: f32| {
                if a > b {
                    (a + (b - a) * x).max(b)
                } else {
                    (a + (b - a) * x).min(b)
                }
            };
            for animation in self.entities.iter_mut().map(|e| &mut e.animation) {
                animation.x = clamped_lerp(animation.x, 0.0, delta_seconds * 20.0);
                animation.y = clamped_lerp(animation.y, 0.0, delta_seconds * 20.0);
            }
        }
        self.animation_timer += delta_seconds;
    }

    pub fn render(&self, ctx: &mut GraphicsContext, _font: &Font, tileset: &Spritesheet) {
        let tile_size = 32.0;
        let drawable_width = ctx.width - crate::ui::UI_AREA_WIDTH;
        let drawable_height = ctx.height;
        let offset = {
            let player = &self.entities[0];
            let focus_x = player.position.x as f32 + player.animation.x + 0.5;
            let focus_y = player.position.y as f32 + player.animation.y + 0.5;
            (
                drawable_width / 2.0 - focus_x * tile_size,
                drawable_height / 2.0 - focus_y * tile_size,
            )
        };
        for (position, sprite, animation) in self
            .entities
            .iter()
            .map(|e| (&e.position, &e.sprite, &e.animation))
        {
            tileset
                .draw(ctx)
                .coordinates((
                    (position.x as f32 + animation.x) * tile_size + offset.0,
                    (position.y as f32 + animation.y) * tile_size + offset.1,
                    tile_size,
                    tile_size,
                ))
                .texture_coordinates(sprite.data[0])
                .z(0.0)
                .finish();
        }

        for (position, animation, health) in self.entities.iter().filter_map(|e| {
            e.health
                .as_ref()
                .map(|health| (&e.position, &e.animation, health))
        }) {
            let pos = (
                (position.x as f32 + animation.x) * tile_size + offset.0,
                (position.y as f32 + animation.y) * tile_size + offset.1,
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
    (x, y): (f32, f32),
    tile_size: f32,
    tint: (f32, f32, f32, f32),
    heart_quarters: i32,
) {
    let hearts_total = (heart_quarters as f32 / 4.0).ceil() as i32;
    let rows = (hearts_total as f32 / 3.0).ceil() as i32;
    for i in 0..hearts_total {
        let coords = (
            x + tile_size / 4.0 * (i % 3) as f32 + tile_size / 8.0,
            y - tile_size / 4.0 * rows as f32 + tile_size / 4.0 * (i / 3) as f32,
            tile_size / 4.0,
            tile_size / 4.0,
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
