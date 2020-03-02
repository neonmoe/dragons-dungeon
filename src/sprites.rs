pub type SpriteData = (i32, i32, i32, i32);

pub const PLAYER: SpriteData = (0, 0, 16, 16);
pub const WALL: SpriteData = (0, 16, 16, 16);
pub const SKELETON: SpriteData = (0, 48, 16, 16);
pub const ICONS_HEART: [SpriteData; 4] =
    [(0, 32, 8, 8), (8, 32, 8, 8), (0, 40, 8, 8), (8, 40, 8, 8)];
pub const ITEM_SWORD: SpriteData = (0, 64, 16, 16);
pub const ITEM_SCYTHE: SpriteData = (0, 80, 16, 16);
pub const ITEM_HAMMER: SpriteData = (0, 96, 16, 16);
pub const ITEM_DAGGER: SpriteData = (0, 112, 16, 16);
pub const ITEM_SHIELD: SpriteData = (0, 128, 16, 16);
pub const ITEM_VAMPIRE_TEETH: SpriteData = (0, 144, 16, 16);
pub const ITEM_STOPWATCH: SpriteData = (0, 160, 16, 16);
