use crate::world::entity::Position;
use crate::world::EntityIter;

#[derive(Debug, Clone)]
pub enum Ai {
    Skeleton(SkeletonAi),
    //Cobweb,
    //Zombie,
    //Dragon,
}

#[derive(Debug, Clone)]
pub struct SkeletonAi {
    step: i32,
}

impl SkeletonAi {
    pub fn new() -> SkeletonAi {
        SkeletonAi { step: 0 }
    }

    pub fn update(&mut self, position: &mut Position, others: EntityIter) {
        match self.step {
            0 => super::move_entity(position, others, 1, 0),
            1 => super::move_entity(position, others, 0, 1),
            2 => super::move_entity(position, others, -1, 0),
            3 => super::move_entity(position, others, 0, -1),
            _ => false,
        };
        self.step = (self.step + 1) % 4;
    }
}
