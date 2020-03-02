mod ai;
mod entities;
mod entity;

use crate::sprites;
use ai::Ai;
use entities::*;
use entity::{AnimationState, Damage, Entity, Health, Inventory, Item, Position};

use fae::{Font, GraphicsContext, Spritesheet};

/// Represents an iterator over all entities except for one. Used when
/// running updates for a that one entity, if it needs to interact
/// with others.
///
/// Created internally by World::split_entities.
pub type EntityIter<'a> =
    std::iter::Chain<std::slice::IterMut<'a, Entity>, std::slice::IterMut<'a, Entity>>;

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
                PROTO_PLAYER.clone_at(0, 0),
                PROTO_WALL.clone_at(1, 2),
                PROTO_SKELETON.clone_at(5, 5),
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
            if entity.is_alive() {
                if let Some(ai) = &mut entity.ai {
                    match ai {
                        Ai::Skeleton(ai) => ai.update(&mut entity.position, others),
                    }
                }
            }
        }
    }

    fn update_player(&mut self, action: PlayerAction) {
        let mut move_direction = None;
        match action {
            PlayerAction::MoveUp => move_direction = Some((0, -1)),
            PlayerAction::MoveDown => move_direction = Some((0, 1)),
            PlayerAction::MoveRight => move_direction = Some((1, 0)),
            PlayerAction::MoveLeft => move_direction = Some((-1, 0)),
            PlayerAction::Wait => {}
        };
        if let Some((xd, yd)) = move_direction {
            let (player, others) = self.split_entities(0);
            let moved = move_entity(&mut player.position, others, xd, yd);

            if !moved {
                let (player, others) = self.split_entities(0);
                attack_direction(
                    &player.position,
                    player.damage.as_ref().unwrap(),
                    player.health.as_mut().unwrap(),
                    player.inventory.as_ref().unwrap(),
                    others,
                    xd,
                    yd,
                )
            }
        }
    }

    pub fn animate(&mut self, delta_seconds: f32, round_duration: f32) {
        let previous_round_entities = match &self.previous_round_entities {
            Some(entities) => entities,
            None => return,
        };
        let progress = if round_duration > 0.0 {
            (self.animation_timer / round_duration).min(1.0)
        } else {
            1.0
        };

        // TODO: Make the player animate before everyone else.

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
                current.animation.x.from = current.animation.x.current - xd as f32;
                current.animation.y.from = current.animation.y.current - yd as f32;

                let alive_opacity = 1.0;
                let alive_rotation = 0.0;
                let dead_opacity = 0.4;
                let dead_rotation = std::f32::consts::PI * 0.4;
                if current.is_alive() {
                    current.animation.opacity.from = alive_opacity;
                    current.animation.opacity.to = alive_opacity;
                    current.animation.rotation.from = alive_rotation;
                    current.animation.rotation.to = alive_rotation;
                    if !previous.is_alive() {
                        // Just revived!
                        current.animation.opacity.from = dead_opacity;
                        current.animation.rotation.from = dead_rotation;
                    }
                } else {
                    current.animation.opacity.from = dead_opacity;
                    current.animation.opacity.to = dead_opacity;
                    current.animation.rotation.from = dead_rotation;
                    current.animation.rotation.to = dead_rotation;
                    if previous.is_alive() {
                        // Just died!
                        current.animation.opacity.from = alive_opacity;
                        current.animation.rotation.from = alive_rotation;
                    }
                }
            }
        }

        let clamped_lerp = |animation: &mut AnimationState<f32>, x: f32| {
            animation.current = if animation.from > animation.to {
                (animation.from + (animation.to - animation.from) * x).max(animation.to)
            } else {
                (animation.from + (animation.to - animation.from) * x).min(animation.to)
            };
        };
        for animation in self.entities.iter_mut().map(|e| &mut e.animation) {
            clamped_lerp(&mut animation.x, progress);
            clamped_lerp(&mut animation.y, progress);
            clamped_lerp(&mut animation.rotation, progress);
            clamped_lerp(&mut animation.opacity, progress);
        }

        self.animation_timer += delta_seconds;
    }

    pub fn render(&self, ctx: &mut GraphicsContext, _font: &Font, tileset: &Spritesheet) {
        let tile_size = 32.0;
        let drawable_width = ctx.width - crate::ui::UI_AREA_WIDTH;
        let drawable_height = ctx.height;
        let offset = {
            let player = &self.entities[0];
            let focus_x = player.position.x as f32 + player.animation.x.current + 0.5;
            let focus_y = player.position.y as f32 + player.animation.y.current + 0.5;
            (
                drawable_width / 2.0 - focus_x * tile_size,
                drawable_height / 2.0 - focus_y * tile_size,
            )
        };

        for (position, sprite, animation, is_alive) in self
            .entities
            .iter()
            .map(|e| (&e.position, &e.sprite, &e.animation, e.is_alive()))
        {
            tileset
                .draw(ctx)
                .coordinates((
                    (position.x as f32 + animation.x.current) * tile_size + offset.0,
                    (position.y as f32 + animation.y.current) * tile_size + offset.1,
                    tile_size,
                    tile_size,
                ))
                .texture_coordinates(sprite.0[0])
                .color((1.0, 1.0, 1.0, animation.opacity.current))
                .rotation(animation.rotation.current, tile_size / 2.0, tile_size / 2.0)
                .z(if is_alive { 0.1 } else { 0.0 })
                .finish();
        }

        for (position, animation, health) in self
            .entities
            .iter()
            .filter(|e| e.is_alive())
            .filter_map(|e| {
                e.health
                    .as_ref()
                    .map(|health| (&e.position, &e.animation, health))
            })
        {
            let pos = (
                (position.x as f32 + animation.x.current) * tile_size + offset.0,
                (position.y as f32 + animation.y.current) * tile_size + offset.1,
            );
            let dark = (0.2, 0.5, 0.8, 0.8 * animation.opacity.current);
            let light = (0.9, 0.1, 0.0, 1.0 * animation.opacity.current);
            draw_hearts(ctx, tileset, pos, tile_size, dark, health.max);
            draw_hearts(ctx, tileset, pos, tile_size, light, health.current);
        }
    }

    fn split_entities(&mut self, separated_index: usize) -> (&mut Entity, EntityIter) {
        let (head, tail) = self.entities.split_at_mut(separated_index);
        let (separated, tail) = tail.split_at_mut(1);
        (&mut separated[0], head.iter_mut().chain(tail.iter_mut()))
    }
}

fn move_entity(position: &mut Position, others: EntityIter, xd: i32, yd: i32) -> bool {
    let (new_x, new_y) = (position.x + xd, position.y + yd);
    for other in others.filter(|e| e.denies_movement && e.is_alive()) {
        if new_x == other.position.x && new_y == other.position.y {
            return false;
        }
    }
    position.x = new_x;
    position.y = new_y;
    true
}

fn attack_direction(
    position: &Position,
    damage: &Damage,
    health: &mut Health,
    inventory: &Inventory,
    others: EntityIter,
    xd: i32,
    yd: i32,
) {
    let (target_x, target_y) = (position.x + xd, position.y + yd);
    let base_damage = calculate_outgoing_damage(damage, inventory);
    let mut damage_dealt = 0;
    for target in others
        .filter(|e| e.position.x == target_x && e.position.y == target_y && e.health.is_some())
    {
        let damage_taken = calculate_incoming_damage(base_damage, &target.inventory);
        if let Some(ref mut health) = target.health {
            let previous_health = health.current;
            health.current = (health.current - damage_taken).max(0);
            damage_dealt += previous_health - health.current;
        }
    }
    if inventory.has_item(Item::VampireTeeth) {
        health.current = (health.current + damage_dealt).min(health.max);
    }
}

fn calculate_outgoing_damage(damage: &Damage, inventory: &Inventory) -> i32 {
    if inventory.has_item(Item::Sword) {
        damage.0 * 2
    } else {
        damage.0
    }
}

fn calculate_incoming_damage(mut base_damage: i32, defender_inventory: &Option<Inventory>) -> i32 {
    if let Some(inv) = defender_inventory {
        if inv.has_item(Item::Shield) && base_damage > 1 {
            base_damage /= 2;
        }
    }
    base_damage
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
