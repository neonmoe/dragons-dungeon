pub type SpriteData = [(i32, i32, i32, i32); 2];

pub const PLAYER: SpriteData = [(0, 0, 64, 64), (0, 0, 64, 64)];
pub const WALL: SpriteData = [(0, 64, 64, 64), (0, 64, 64, 64)];
pub const SKELETON: SpriteData = [(0, 192, 64, 64), (0, 0, 64, 64)];
pub const ICONS_HEART: [SpriteData; 4] = [
    [(0, 128, 16, 16), (0, 128, 16, 16)],
    [(16, 128, 16, 16), (16, 128, 16, 16)],
    [(32, 128, 16, 16), (32, 128, 16, 16)],
    [(48, 128, 16, 16), (48, 128, 16, 16)],
];
