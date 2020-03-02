use crate::world::entity::Position;
use crate::world::EntityIter;

#[derive(Debug, Clone)]
pub enum Ai {
    Skeleton(SkeletonAi),
    //Cobweb,
    //Zombie,
    //Dragon,
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

    pub fn update(&mut self, position: &mut Position, others: EntityIter) {
        let (xd, yd) = SKELETON_PATH[self.step];
        super::move_entity(position, others, xd, yd);
        self.step = (self.step + 1) % SKELETON_PATH.len();
    }
}
