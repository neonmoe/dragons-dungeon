// Might implement rebindable keys later, but currently this just
// contains the default bindings in a very hardcoded way.

use sdl2::keyboard::Keycode;

const KEYS_MOVE_UP: [Keycode; 3] = [Keycode::Up, Keycode::W, Keycode::K];
const KEYS_MOVE_DOWN: [Keycode; 3] = [Keycode::Down, Keycode::S, Keycode::J];
const KEYS_MOVE_LEFT: [Keycode; 3] = [Keycode::Left, Keycode::A, Keycode::H];
const KEYS_MOVE_RIGHT: [Keycode; 3] = [Keycode::Right, Keycode::D, Keycode::L];

pub fn is_key_move_up(keycode: Keycode) -> bool {
    KEYS_MOVE_UP.contains(&keycode)
}

pub fn is_key_move_down(keycode: Keycode) -> bool {
    KEYS_MOVE_DOWN.contains(&keycode)
}

pub fn is_key_move_left(keycode: Keycode) -> bool {
    KEYS_MOVE_LEFT.contains(&keycode)
}

pub fn is_key_move_right(keycode: Keycode) -> bool {
    KEYS_MOVE_RIGHT.contains(&keycode)
}
