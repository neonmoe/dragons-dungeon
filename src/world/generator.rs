use super::entities::*;
use super::entity::Entity;
use super::Room;
use rand_core::SeedableRng;
use rand_pcg::Pcg32;

const ROOM_WIDTH: i32 = 8;
const ROOM_HEIGHT: i32 = 6;

enum RoomType {
    StartRoom,
    ItemRoom { count: i32 },
    MonsterRoom { count: i32 },
    StairsRoom,
    BossRoom,
}

#[derive(Debug, Clone)]
pub struct WorldGenerator {
    rng: Pcg32,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> WorldGenerator {
        WorldGenerator {
            rng: Pcg32::seed_from_u64(seed),
        }
    }

    pub fn generate(&mut self, _level: i32) -> (Vec<Entity>, Vec<Room>) {
        let mut entities = Vec::with_capacity(128);
        let mut rooms = Vec::with_capacity(8);
        let cursor = (0, 0);
        rooms.push(self.generate_room(&mut entities, cursor, RoomType::StartRoom));
        (entities, rooms)
    }

    fn generate_room(
        &mut self,
        entities: &mut Vec<Entity>,
        (x, y): (i32, i32),
        _room_type: RoomType,
    ) -> Room {
        let room_x = x - ROOM_WIDTH / 2;
        let room_y = y - ROOM_HEIGHT / 2;
        for y in 0..ROOM_HEIGHT {
            for x in 0..ROOM_WIDTH {
                let border = x == 0 || y == 0 || x == ROOM_WIDTH - 1 || y == ROOM_HEIGHT - 1;
                let world_x = x + room_x;
                let world_y = y + room_y;
                if border {
                    entities.push(PROTO_WALL.clone_at(world_x, world_y));
                }
            }
        }
        Room {
            x: room_x,
            y: room_y,
            width: ROOM_WIDTH,
            height: ROOM_HEIGHT,
        }
    }
}
