use super::entities::*;
use super::entity::Entity;
use super::Room;
use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;

const ROOM_WIDTH: i32 = 11;
const ROOM_HEIGHT: i32 = 7;

fn door_left_of((x, y): (i32, i32)) -> Door {
    Door::new(x, y + ROOM_HEIGHT / 2, x - ROOM_WIDTH, y, (1, 0))
}

#[derive(Debug, Clone, Copy)]
enum RoomType {
    StartRoom,
    ItemRoom,
    MonsterRoom { count: i32 },
    StairsRoom,
    BossRoom,
}

fn get_rooms(level: i32) -> [RoomType; 8] {
    use RoomType::*;
    [
        StartRoom,
        ItemRoom,
        MonsterRoom { count: 1 },
        if level >= 4 { BossRoom } else { StairsRoom },
        MonsterRoom { count: level },
        MonsterRoom { count: 2 },
        ItemRoom,
        MonsterRoom {
            count: level * 3 / 2,
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct Door {
    x: i32,
    y: i32,
    room_x: i32,
    room_y: i32,
    direction: (i32, i32),
}

impl Door {
    const fn new(x: i32, y: i32, room_x: i32, room_y: i32, direction: (i32, i32)) -> Door {
        Door {
            x,
            y,
            room_x,
            room_y,
            direction,
        }
    }
}

fn rand_range(rng: &mut Pcg32, min: i32, max: i32) -> i32 {
    min + (rng.next_u32() as i32).abs() % (max - min)
}

fn rand_enemy(rng: &mut Pcg32, level: i32) -> Entity {
    let r = rng.next_u32() % 100;
    if level > 3 {
        if r < 60 {
            PROTO_SKELETON.clone()
        } else {
            PROTO_ZOMBIE.clone()
        }
    } else if level > 2 {
        if r < 30 {
            PROTO_SKELETON.clone()
        } else {
            PROTO_ZOMBIE.clone()
        }
    } else {
        PROTO_ZOMBIE.clone()
    }
}

fn rand_item(rng: &mut Pcg32, level: i32) -> Entity {
    let index = if level > 2 {
        (rng.next_u32() as usize) % PROTO_ITEMS.len()
    } else {
        (rng.next_u32() as usize) % 4
    };
    PROTO_ITEMS[index].clone()
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

    pub fn generate(&mut self, level: i32) -> (Vec<Entity>, Vec<Room>, (i32, i32)) {
        let origin = (0, 0);
        let room_types = get_rooms(level);
        let mut entities: Vec<Entity> = Vec::with_capacity(128);
        let mut rooms = Vec::with_capacity(room_types.len());
        let mut doors: Vec<Door> = Vec::with_capacity(rooms.len() * 4);
        for room_type in &room_types[0..4 + level.min(4).max(0) as usize] {
            let door_index = self.rng.next_u32() as usize % doors.len().max(1);
            let door = if doors.len() > door_index {
                let door = doors
                    .splice(door_index..door_index + 1, None)
                    .nth(0)
                    .unwrap();
                entities.push(PROTO_DOOR.clone_at(door.x, door.y));
                door
            } else {
                door_left_of(origin)
            };

            let room_x = door.room_x + door.direction.0 * (ROOM_WIDTH - 1);
            let room_y = door.room_y + door.direction.1 * (ROOM_HEIGHT - 1);

            let (room, new_doors) =
                self.generate_room(&mut entities, &rooms, room_x, room_y, level, *room_type);
            rooms.push(room);
            doors.extend(new_doors.into_iter());
        }

        // Seal up the remaining doors
        for door in doors {
            entities.push(PROTO_WALL.clone_at(door.x, door.y));
        }

        (entities, rooms, origin)
    }

    fn generate_room(
        &mut self,
        entities: &mut Vec<Entity>,
        rooms: &[Room],
        room_x: i32,
        room_y: i32,
        level: i32,
        room_type: RoomType,
    ) -> (Room, Vec<Door>) {
        let room_above = rooms
            .iter()
            .find(|room| room.contains(room_x + ROOM_WIDTH / 2, room_y - ROOM_HEIGHT / 2))
            .is_some();
        let room_below = rooms
            .iter()
            .find(|room| room.contains(room_x + ROOM_WIDTH / 2, room_y + ROOM_HEIGHT * 3 / 2))
            .is_some();
        let room_left = rooms
            .iter()
            .find(|room| room.contains(room_x - ROOM_WIDTH / 2, room_y + ROOM_HEIGHT / 2))
            .is_some();
        let room_right = rooms
            .iter()
            .find(|room| room.contains(room_x + ROOM_WIDTH * 3 / 2, room_y + ROOM_HEIGHT / 2))
            .is_some();

        let mut doors = Vec::with_capacity(4);
        for y in 0..ROOM_HEIGHT {
            for x in 0..ROOM_WIDTH {
                let world_x = x + room_x;
                let world_y = y + room_y;

                let mut border = false;
                let mut door = false;
                let mut try_door = |direction: (i32, i32)| {
                    if x == ROOM_WIDTH / 2 || y == ROOM_HEIGHT / 2 {
                        doors.push(Door::new(world_x, world_y, room_x, room_y, direction));
                        door = true;
                    } else {
                        border = true;
                    }
                };

                let x_corner = x == 0 || x == ROOM_WIDTH - 1;
                let y_corner = y == 0 || y == ROOM_HEIGHT - 1;
                if !(x_corner && y_corner) {
                    if x == 0 && !room_left {
                        try_door((-1, 0));
                    }
                    if x == ROOM_WIDTH - 1 && !room_right {
                        try_door((1, 0));
                    }
                    if y == 0 && !room_above {
                        try_door((0, -1));
                    }
                    if y == ROOM_HEIGHT - 1 && !room_below {
                        try_door((0, 1));
                    }
                }

                if border {
                    entities.push(PROTO_WALL.clone_at(world_x, world_y));
                }
            }
        }

        let min_x = room_x + 2;
        let max_x = room_x + ROOM_WIDTH - 3;
        let min_y = room_y + 2;
        let max_y = room_y + ROOM_HEIGHT - 3;
        let center_x = room_x + ROOM_WIDTH / 2;
        let center_y = room_y + ROOM_HEIGHT / 2;
        match room_type {
            RoomType::MonsterRoom { count } => {
                for _ in 0..count {
                    let mut enemy = rand_enemy(&mut self.rng, level);
                    enemy.position.x = rand_range(&mut self.rng, min_x, max_x);
                    enemy.position.y = rand_range(&mut self.rng, min_y, max_y);
                    // TODO: Check that this position is not occupied
                    entities.push(enemy);
                }
            }
            RoomType::ItemRoom => {
                let mut item = rand_item(&mut self.rng, level);
                item.position.x = center_x;
                item.position.y = center_y;
                entities.push(item);
            }
            RoomType::StartRoom => {}
            RoomType::StairsRoom => {
                entities.push(PROTO_NEXT_LEVEL.clone_at(center_x, center_y));
            }
            RoomType::BossRoom => {
                entities.push(PROTO_DRAGON.clone_at(center_x, center_y));
            }
        }

        if self.rng.next_u32() % 3 == 0 {
            let x = rand_range(&mut self.rng, min_x, max_x);
            let y = rand_range(&mut self.rng, min_y, max_y);
            entities.push(PROTO_APPLE.clone_at(x, y));
        }

        (
            Room {
                x: room_x,
                y: room_y,
                width: ROOM_WIDTH,
                height: ROOM_HEIGHT,
            },
            doors,
        )
    }
}
