use raylib::prelude::*;
use std::collections::HashMap;
use crate::snake::Snake;

#[derive(Eq, PartialEq)]
pub enum GameState {
    Paused,
    Running,
    Over
}

pub const LEFT: [i32; 2] = [-1, 0];
pub const RIGHT: [i32; 2] = [1, 0];
pub const UP: [i32; 2] = [0, -1];
pub const DOWN: [i32; 2] = [0, 1];
pub const IDLE: [i32; 2] = [0, 0];


pub fn direction_hash() -> HashMap<KeyboardKey, [i32; 2]> {
    vec![
        (KeyboardKey::KEY_A, LEFT),
        (KeyboardKey::KEY_D, RIGHT),
        (KeyboardKey::KEY_W, UP),
        (KeyboardKey::KEY_S, DOWN),
        (KeyboardKey::KEY_LEFT, LEFT),
        (KeyboardKey::KEY_RIGHT, RIGHT),
        (KeyboardKey::KEY_UP, UP),
        (KeyboardKey::KEY_DOWN, DOWN)
    ]
    .into_iter()
    .collect()
}


pub fn rot_hash(snake: &Snake) -> HashMap<[i32; 2], (i32, (f32, f32))> {
    vec![
        (UP, (90, (snake.head_texture.width() as f32, 0_f32))),
        (DOWN, (270, (-0_f32, snake.head_texture.height() as f32))),
        (LEFT, (0, (0_f32, 0_f32))),
        (RIGHT, (180, (snake.head_texture.width() as f32, snake.head_texture.height() as f32))),
        (IDLE, (0, (0_f32, 0_f32)))
    ]
    .into_iter()
    .collect()
}
