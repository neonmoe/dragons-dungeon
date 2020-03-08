pub type SpriteData = (i32, i32, i32, i32);

pub const PLAYER: SpriteData = (0 * 2, 0 * 2, 16 * 2, 16 * 2);
pub const WALL: SpriteData = (0 * 2, 16 * 2, 16 * 2, 16 * 2);
pub const SKELETON: SpriteData = (0 * 2, 48 * 2, 16 * 2, 16 * 2);
pub const ZOMBIE: SpriteData = (0 * 2, 192 * 2, 16 * 2, 16 * 2);
pub const DRAGON: SpriteData = (0 * 2, 208 * 2, 16 * 2, 16 * 2);
pub const FLAME: SpriteData = (0 * 2, 256 * 2, 16 * 2, 16 * 2);
pub const DOOR: SpriteData = (0 * 2, 240 * 2, 16 * 2, 16 * 2);
pub const NEXT_LEVEL: SpriteData = (0 * 2, 272 * 2, 16 * 2, 16 * 2);
pub const ICONS_HEART: [SpriteData; 4] = [
    (0 * 2, 32 * 2, 6 * 2, 6 * 2),
    (6 * 2, 32 * 2, 6 * 2, 6 * 2),
    (0 * 2, 38 * 2, 6 * 2, 6 * 2),
    (6 * 2, 38 * 2, 6 * 2, 6 * 2),
];
pub const ITEM_SWORD: SpriteData = (0 * 2, 64 * 2, 16 * 2, 16 * 2);
pub const ITEM_SCYTHE: SpriteData = (0 * 2, 80 * 2, 16 * 2, 16 * 2);
pub const ITEM_HAMMER: SpriteData = (0 * 2, 96 * 2, 16 * 2, 16 * 2);
pub const ITEM_DAGGER: SpriteData = (0 * 2, 112 * 2, 16 * 2, 16 * 2);
pub const ITEM_SHIELD: SpriteData = (0 * 2, 128 * 2, 16 * 2, 16 * 2);
pub const ITEM_VAMPIRE_TEETH: SpriteData = (0 * 2, 144 * 2, 16 * 2, 16 * 2);
pub const ITEM_STOPWATCH: SpriteData = (0 * 2, 160 * 2, 16 * 2, 16 * 2);
pub const ITEM_APPLE: SpriteData = (0 * 2, 224 * 2, 16 * 2, 16 * 2);

#[allow(dead_code)]
pub const COBWEB: SpriteData = (0 * 2, 176 * 2, 16 * 2, 16 * 2);
