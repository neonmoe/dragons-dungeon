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
}

impl AiTrait for SkeletonAi {
    fn update(&mut self, index: usize, entities: &mut [Entity]) {
        let player_direction = {
            let player = &entities[0].position;
            let me = &entities[index].position;
            let (xd, yd) = (player.x - me.x, player.y - me.y);
            if (xd == 0 && yd.abs() == 1) || (xd.abs() == 1 && yd == 0) {
                Some((xd, yd))
            } else {
                None
            }
        };
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

#[derive(Debug, Clone)]
pub struct ZombieAi {}

impl ZombieAi {
    pub const fn new() -> ZombieAi {
        ZombieAi {}
    }
}

impl AiTrait for ZombieAi {
    fn update(&mut self, _index: usize, _entities: &mut [Entity]) {}
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
}
