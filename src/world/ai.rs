use crate::world::entity::Entity;
use crate::world::{attack_direction, move_entity, split_entities};

#[derive(Debug, Clone)]
pub enum Ai {
    Skeleton,
    //Cobweb,
    Zombie,
    Dragon,
}

impl Ai {
    pub fn create_ai(&self) -> Box<dyn AiTrait> {
        match self {
            Ai::Skeleton => Box::new(SkeletonAi::new()),
            Ai::Zombie => Box::new(ZombieAi::new()),
            Ai::Dragon => Box::new(DragonAi::new()),
        }
    }
}

pub trait AiTrait {
    fn update(&mut self, index: usize, entities: &mut [Entity]);
    fn animation_state(&self, index: usize, entities: &[Entity]) -> i32;
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

fn find_path_to_player(
    ai_index: usize,
    entities: &mut [Entity],
    radius: i32,
) -> Option<(i32, i32)> {
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
    fn update(&mut self, index: usize, entities: &mut [Entity]) {
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
    }

    fn animation_state(&self, index: usize, entities: &[Entity]) -> i32 {
        if self.scared(index, entities) {
            2
        } else if find_player(index, entities).is_some() {
            1
        } else {
            0
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
    fn update(&mut self, index: usize, entities: &mut [Entity]) {
        if self.exhausted {
            self.exhausted = false;
            return;
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
    }

    fn animation_state(&self, index: usize, entities: &[Entity]) -> i32 {
        if !self.exhausted || !entities[index].is_alive() {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub struct DragonAi {}

impl DragonAi {
    pub const fn new() -> DragonAi {
        DragonAi {}
    }
}

impl AiTrait for DragonAi {
    fn update(&mut self, _index: usize, _entities: &mut [Entity]) {}

    fn animation_state(&self, _index: usize, _entities: &[Entity]) -> i32 {
        0
    }
}
