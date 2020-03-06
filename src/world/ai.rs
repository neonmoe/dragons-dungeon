use crate::world::entity::Entity;
use crate::world::{attack_direction, entities, move_entity, split_entities};

#[derive(Debug, Clone)]
pub enum Ai {
    Skeleton,
    //Cobweb,
    Zombie,
    Dragon,
    Flame,
}

impl Ai {
    pub fn create_ai(&self) -> Box<dyn AiTrait> {
        match self {
            Ai::Skeleton => Box::new(SkeletonAi::new()),
            Ai::Zombie => Box::new(ZombieAi::new()),
            Ai::Dragon => Box::new(DragonAi::new()),
            Ai::Flame => Box::new(FlameAi::new()),
        }
    }
}

pub trait AiTrait {
    fn update(&mut self, index: usize, entities: &mut [Entity]) -> Option<Vec<Entity>>;
    fn animation_state(&self, index: usize, entities: &[Entity]) -> (i32, bool);
}

fn find_player(ai_index: usize, entities: &[Entity]) -> Option<(i32, i32)> {
    let player = &entities[0].position;
    let me = &entities[ai_index].position;
    let (xd, yd) = (player.x - me.x, player.y - me.y);
    if (xd == 0 && yd.abs() == 1) || (xd.abs() == 1 && yd == 0) {
        Some((xd, yd))
    } else {
        None
    }
}

fn find_path_to_player(ai_index: usize, entities: &[Entity], radius: i32) -> Option<(i32, i32)> {
    let player = &entities[0].position;
    let me = &entities[ai_index].position;
    let (xd, yd) = (player.x - me.x, player.y - me.y);
    if xd.abs().max(yd.abs()) <= radius {
        let xd_ = if xd == 0 { 0 } else { xd.abs() / xd };
        let yd_ = if yd == 0 { 0 } else { yd.abs() / yd };
        if xd.abs() > yd.abs() {
            Some((xd_, 0))
        } else {
            Some((0, yd_))
        }
    } else {
        None
    }
}

const SKELETON_PATH: [(i32, i32); 8] = [
    (1, 0),
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (0, 1),
];

#[derive(Debug, Clone)]
pub struct SkeletonAi {
    step: usize,
}

impl SkeletonAi {
    pub const fn new() -> SkeletonAi {
        SkeletonAi { step: 0 }
    }

    fn scared(&self, index: usize, entities: &[Entity]) -> bool {
        entities[index]
            .health
            .iter()
            .any(|h| h.current <= h.max / 2)
    }
}

impl AiTrait for SkeletonAi {
    fn update(&mut self, index: usize, entities: &mut [Entity]) -> Option<Vec<Entity>> {
        if self.scared(index, entities) {
            let player_path = find_path_to_player(index, entities, 2);
            if let Some((xd, yd)) = player_path {
                let (me, others) = split_entities(index, entities);
                move_entity(&mut me.position, others, -xd, -yd);
            }
        } else {
            let player_direction = find_player(index, entities);
            let (me, others) = split_entities(index, entities);
            if let Some((xd, yd)) = player_direction {
                attack_direction(
                    &me.position,
                    me.damage.as_ref().unwrap(),
                    &mut me.health,
                    &me.inventory,
                    others,
                    xd,
                    yd,
                );
            } else {
                let (xd, yd) = SKELETON_PATH[self.step];
                move_entity(&mut me.position, others, xd, yd);
                self.step = (self.step + 1) % SKELETON_PATH.len();
            }
        }
        None
    }

    fn animation_state(&self, index: usize, entities: &[Entity]) -> (i32, bool) {
        if self.scared(index, entities) {
            (2, false)
        } else if find_player(index, entities).is_some() {
            (1, false)
        } else {
            (0, false)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZombieAi {
    exhausted: bool,
}

impl ZombieAi {
    pub const fn new() -> ZombieAi {
        ZombieAi { exhausted: false }
    }
}

impl AiTrait for ZombieAi {
    fn update(&mut self, index: usize, entities: &mut [Entity]) -> Option<Vec<Entity>> {
        if self.exhausted {
            self.exhausted = false;
            return None;
        }

        let direction_attack = find_player(index, entities);
        let direction_path = find_path_to_player(index, entities, 4);
        let (me, others) = split_entities(index, entities);
        if let Some((xd, yd)) = direction_attack {
            attack_direction(
                &me.position,
                me.damage.as_ref().unwrap(),
                &mut me.health,
                &me.inventory,
                others,
                xd,
                yd,
            );
        } else {
            if let Some((xd, yd)) = direction_path {
                move_entity(&mut me.position, others, xd, yd);
            }
        }

        self.exhausted = true;
        None
    }

    fn animation_state(&self, index: usize, entities: &[Entity]) -> (i32, bool) {
        if !self.exhausted || !entities[index].is_alive() {
            (1, true)
        } else {
            (0, false)
        }
    }
}

#[derive(Debug, Clone)]
pub struct DragonAi {
    charge_direction: Option<(i32, i32, i32)>,
    flame_stage: Option<(i32, i32)>,
}

impl DragonAi {
    pub const fn new() -> DragonAi {
        DragonAi {
            charge_direction: None,
            flame_stage: None,
        }
    }
}

impl AiTrait for DragonAi {
    fn update(&mut self, index: usize, entities: &mut [Entity]) -> Option<Vec<Entity>> {
        let mut spawns = None;
        if let Some((xd, yd, ref mut time)) = self.charge_direction {
            let (me, others) = split_entities(index, entities);
            let moved = move_entity(&mut me.position, others, xd, yd);
            *time -= 1;
            if !moved || *time == 0 {
                self.charge_direction = None;
            }
            if !moved {
                // Something was blocking the way, try to attack
                let (me, others) = split_entities(index, entities);
                attack_direction(
                    &me.position,
                    me.damage.as_ref().unwrap(),
                    &mut me.health,
                    &me.inventory,
                    others,
                    xd,
                    yd,
                );
            }
        } else if let Some((ref mut flame, _)) = self.flame_stage {
            *flame += 1;
            if *flame == FLAME_LIFETIME - 1 {
                self.flame_stage = None;
            }
        } else {
            let mut next_strategy_chosen = false;
            if entities[index].health.iter().any(|h| h.current > h.max / 2) {
                if let Some((xd, yd)) = find_path_to_player(index, entities, 4) {
                    self.charge_direction = Some((xd, yd, 5));
                    next_strategy_chosen = true;
                }
            }
            if !next_strategy_chosen {
                // Didn't charge, start spitting flame:
                let direction = if entities[0].position.x < entities[index].position.x {
                    -1
                } else {
                    1
                };
                self.flame_stage = Some((0, direction));
                let mut flames = Vec::with_capacity(9);
                let dragon = &entities[index].position;
                let offset_x = if direction == -1 { -3 } else { 0 };
                let offset_y = -2;
                for y in 0..5 {
                    for x in 0..4 {
                        let x = x + dragon.x + offset_x;
                        let y = y + dragon.y + offset_y;
                        if x == dragon.x && y == dragon.y {
                            continue;
                        }
                        flames.push(entities::PROTO_FLAME.clone_at(x, y));
                    }
                }
                spawns = Some(flames);
            }
        }
        spawns
    }

    fn animation_state(&self, _index: usize, _entities: &[Entity]) -> (i32, bool) {
        if let Some((xd, yd, _)) = self.charge_direction {
            let flip = xd < 0 || yd < 0;
            (1, flip)
        } else if let Some((flame_stage, xd)) = self.flame_stage {
            if flame_stage == FLAME_LIFETIME - 2 {
                (3, xd < 0)
            } else {
                (2, xd < 0)
            }
        } else {
            (0, false)
        }
    }
}

const FLAME_BUILDUP_TICKS: i32 = 3;
const FLAME_LIFETIME: i32 = 7;

#[derive(Debug, Clone)]
pub struct FlameAi {
    state: i32,
}

impl FlameAi {
    const fn new() -> FlameAi {
        FlameAi { state: 0 }
    }
}

impl AiTrait for FlameAi {
    fn update(&mut self, index: usize, entities: &mut [Entity]) -> Option<Vec<Entity>> {
        self.state += 1;
        if self.state > FLAME_BUILDUP_TICKS && self.state < FLAME_LIFETIME - 1 {
            let (me, others) = split_entities(index, entities);
            attack_direction(
                &me.position,
                me.damage.as_ref().unwrap(),
                &mut me.health,
                &me.inventory,
                others,
                0,
                0,
            );
        }
        if self.state >= FLAME_LIFETIME {
            entities[index].marked_for_death = true;
        }
        None
    }

    fn animation_state(&self, _index: usize, _entities: &[Entity]) -> (i32, bool) {
        let flip = (self.state - 1).max(FLAME_BUILDUP_TICKS) % 2 == 0;
        if self.state == FLAME_LIFETIME - 1 {
            (4, flip)
        } else {
            ((self.state - 1).min(FLAME_BUILDUP_TICKS), flip)
        }
    }
}
