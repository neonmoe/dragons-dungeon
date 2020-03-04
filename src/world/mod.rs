mod ai;
mod entities;
mod entity;

use crate::{layers, sprites};
use ai::Ai;
use entities::*;
use entity::{Animation, AnimationState, Damage, Entity, Health, Inventory, Position, Sprite};

use fae::{Font, GraphicsContext, Spritesheet};

pub use entity::Item;

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
    Pickup,
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
        let mut entities = Vec::new();
        entities.push(PROTO_PLAYER.clone_at(0, 0));

        entities.push(PROTO_WALL.clone_at(0, 2));
        entities.push(PROTO_DOOR.clone_at(1, 2));
        entities.push(PROTO_WALL.clone_at(2, 2));

        entities.push(PROTO_SKELETON.clone_at(5, 5));
        entities.push(PROTO_COBWEB.clone_at(3, 4));
        entities.push(PROTO_ZOMBIE.clone_at(0, 4));
        entities.push(PROTO_DRAGON.clone_at(2, 7));

        entities.push(PROTO_APPLE.clone_at(4, 1));
        let mut y = 3;
        for item in PROTO_ITEMS.iter() {
            entities.push(item.clone_at(4, y));
            y += 1;
        }

        World {
            entities,
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

    pub fn player(&self) -> &Entity {
        &self.entities[0]
    }

    /// Excluding the player, get the player with World::player.
    pub fn entities(&self) -> &[Entity] {
        &self.entities[1..]
    }

    fn update_player(&mut self, action: PlayerAction) {
        if !self.entities[0].is_alive() {
            return;
        }

        let mut move_direction = None;
        match action {
            PlayerAction::MoveUp => move_direction = Some((0, -1)),
            PlayerAction::MoveDown => move_direction = Some((0, 1)),
            PlayerAction::MoveRight => move_direction = Some((1, 0)),
            PlayerAction::MoveLeft => move_direction = Some((-1, 0)),
            PlayerAction::Pickup => {
                let (player, others) = self.split_entities(0);
                pickup(&player.position, player.inventory.as_mut().unwrap(), others);
            }
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
                    &player.inventory,
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
        let tile_size = 48.0;
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

        let mut draw_entity =
            |position: &Position, sprite: &Sprite, animation: &Animation, z: f32| {
                let x = (position.x as f32 + animation.x.current) * tile_size + offset.0;
                let y = (position.y as f32 + animation.y.current) * tile_size + offset.1;
                tileset
                    .draw(ctx)
                    .coordinates((x, y, tile_size, tile_size))
                    .texture_coordinates(sprite.0)
                    .color((1.0, 1.0, 1.0, animation.opacity.current))
                    .rotation(animation.rotation.current, tile_size / 2.0, tile_size / 2.0)
                    .z(z)
                    .finish();
            };

        // Draw the dead
        for (position, sprite, animation) in self
            .entities
            .iter()
            .filter(|e| !e.is_alive() && e.visible)
            .map(|e| (&e.position, &e.sprite, &e.animation))
        {
            draw_entity(position, sprite, animation, layers::DEAD);
        }

        // Draw the alive (so they get drawn after the dead
        for (position, sprite, animation) in self
            .entities
            .iter()
            .filter(|e| e.is_alive() && e.visible)
            .map(|e| (&e.position, &e.sprite, &e.animation))
        {
            draw_entity(position, sprite, animation, layers::ALIVE);
        }

        // Draw hearts
        for (position, animation, health) in self
            .entities
            .iter()
            .filter(|e| e.is_alive() && e.visible)
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
            let (current, max) = (health.current, health.max);
            draw_hearts(ctx, tileset, pos, tile_size, dark, max, max);
            draw_hearts(ctx, tileset, pos, tile_size, light, current, max);
        }
    }

    fn split_entities(&mut self, separated_index: usize) -> (&mut Entity, EntityIter) {
        let (head, tail) = self.entities.split_at_mut(separated_index);
        let (separated, tail) = tail.split_at_mut(1);
        (&mut separated[0], head.iter_mut().chain(tail.iter_mut()))
    }
}

fn pickup(position: &Position, inventory: &mut Inventory, others: EntityIter) {
    if let Some(pickup) = others
        .filter(|e| e.position == *position && e.drop.is_some())
        .nth(0)
    {
        if let Some(item) = pickup.drop {
            if let Some(replacing_item) = inventory.add_item(item) {
                pickup.drop = Some(replacing_item);
                pickup.sprite = replacing_item.sprite();
            } else {
                pickup.drop = None;
                pickup.visible = false;
            }
        }
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
    inventory: &Option<Inventory>,
    others: EntityIter,
    xd: i32,
    yd: i32,
) {
    let (target_x, target_y) = (position.x + xd, position.y + yd);
    let mut damage_dealt = 0;
    for target in others
        .filter(|e| e.position.x == target_x && e.position.y == target_y && e.health.is_some())
    {
        if let Some(ref mut target_health) = target.health {
            let damage_taken =
                calculate_damage(damage, inventory, target_health, &target.inventory);
            let previous_target_health = target_health.current;
            target_health.current = (target_health.current - damage_taken).max(0);
            damage_dealt += previous_target_health - target_health.current;
        }
    }
    if inventory.iter().any(|inv| inv.has_item(Item::VampireTeeth)) {
        health.current = (health.current + damage_dealt).min(health.max);
    }
}

fn calculate_damage(
    attacker_damage: &Damage,
    attacker_inventory: &Option<Inventory>,
    defender_health: &Health,
    defender_inventory: &Option<Inventory>,
) -> i32 {
    let mut damage = attacker_damage.0;

    // Apply attacker bonuses
    if let Some(inv) = attacker_inventory {
        if inv.has_item(Item::Sword) {
            damage *= 2;
        }
        if inv.has_item(Item::Scythe) && defender_health.current <= defender_health.max / 2 {
            damage = defender_health.current;
        }
    }

    // Apply defender bonuses
    if let Some(inv) = defender_inventory {
        if inv.has_item(Item::Shield) && damage > 1 {
            damage /= 2;
        }
    }

    damage
}

fn draw_hearts(
    ctx: &mut GraphicsContext,
    tileset: &Spritesheet,
    (x, y): (f32, f32),
    tile_size: f32,
    tint: (f32, f32, f32, f32),
    heart_quarters: i32,
    heart_quarters_max: i32,
) {
    let hearts_per_row = 3;
    let hearts = (heart_quarters as f32 / 4.0).ceil() as i32;
    let hearts_max = (heart_quarters_max as f32 / 4.0).ceil() as i32;
    let rows = (hearts_max as f32 / hearts_per_row as f32).ceil() as i32;
    let heart_size = tile_size * 6.0 / 16.0;
    for i in 0..hearts {
        let index_on_row = (i % hearts_per_row) as f32;
        let horizontal_offset = if rows > 1 {
            tile_size / 2.0 - heart_size * hearts_per_row as f32 / 2.0
        } else {
            tile_size / 2.0 - heart_size * hearts_max as f32 / 2.0
        };
        let coords = (
            x + heart_size * index_on_row + horizontal_offset,
            y - heart_size * rows as f32 + heart_size * (i / hearts_per_row) as f32,
            heart_size,
            heart_size,
        );
        let quarters = (4 - (heart_quarters - i * 4)).max(0) as usize;
        tileset
            .draw(ctx)
            .coordinates(coords)
            .texture_coordinates(sprites::ICONS_HEART[quarters])
            .z(layers::HEARTS)
            .color(tint)
            .finish();
    }
}
