pub type SpriteData = (i32, i32, i32, i32);

pub const PLAYER: SpriteData = (0, 0, 16, 16);
pub const WALL: SpriteData = (0, 16, 16, 16);
pub const SKELETON: SpriteData = (0, 48, 16, 16);
pub const ZOMBIE: SpriteData = (0, 192, 16, 16);
pub const DRAGON: SpriteData = (0, 208, 16, 16);
pub const FLAME: SpriteData = (0, 256, 16, 16);
pub const DOOR: SpriteData = (0, 240, 16, 16);
pub const ICONS_HEART: [SpriteData; 4] =
    [(0, 32, 6, 6), (6, 32, 6, 6), (0, 38, 6, 6), (6, 38, 6, 6)];
pub const ITEM_SWORD: SpriteData = (0, 64, 16, 16);
pub const ITEM_SCYTHE: SpriteData = (0, 80, 16, 16);
pub const ITEM_HAMMER: SpriteData = (0, 96, 16, 16);
pub const ITEM_DAGGER: SpriteData = (0, 112, 16, 16);
pub const ITEM_SHIELD: SpriteData = (0, 128, 16, 16);
pub const ITEM_VAMPIRE_TEETH: SpriteData = (0, 144, 16, 16);
pub const ITEM_STOPWATCH: SpriteData = (0, 160, 16, 16);
pub const ITEM_APPLE: SpriteData = (0, 224, 16, 16);

#[allow(dead_code)]
pub const COBWEB: SpriteData = (0, 176, 16, 16);
