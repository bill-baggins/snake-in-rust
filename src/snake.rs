use crate::raylib_build::GameBuild;
use crate::globals;
use raylib::prelude::*;
use raylib::core::misc::get_random_value as grv;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Snake {
    pub score: i32,
    pub head_texture: Texture2D,
    pub body_texture: Texture2D,
    pub arr: VecDeque<[i32; 2]>,
    pub collected_fruit: bool,
    pub hit_self: bool,
    pub key_hash: HashMap<KeyboardKey, [i32; 2]>,
    rot_hash: HashMap<[i32; 2], (i32, (f32, f32))>,
    starting_length: i32,
    current_direction: [i32; 2],
    count: i32,
    count_limit: i32
}

impl Snake {
    // Creates a new instance of the snake. The snake is controlled by the player.
    pub fn new(head_path: &str, body_path: &str, starting_length: i32, scale: f32, count_limit: i32, gb: &mut GameBuild) -> Snake {
        let mut snake_head = Image::load_image(head_path).expect("Unable to load image: snake_head.png");
        let mut snake_body = Image::load_image(body_path).expect("Unable to load image: snake.png");
        let snake_arr: VecDeque<[i32; 2]> = VecDeque::new();
        let snake_starting_length = starting_length;

        // Resize the image before converting them to textures.
        if scale != 1f32 {
            let head_width = (snake_head.width() as f32 * scale) as i32;
            let head_height = (snake_head.height() as f32 * scale) as i32;
            Image::resize(&mut snake_head, head_width, head_height);
            Image::resize(&mut snake_body, head_width, head_height);
        }

        let snake_head = gb.rl.load_texture_from_image(&gb.thread, &snake_head).expect("Unable to convert head_image to Texture2D");
        let snake_body = gb.rl.load_texture_from_image(&gb.thread, &snake_body).expect("Unable to convert body_image to Texture2D");

        let mut snake_struct = Snake {
            score: 0,
            head_texture: snake_head, 
            body_texture: snake_body, 
            arr: snake_arr, 
            starting_length: snake_starting_length,
            hit_self: false, 
            key_hash: Default::default(),
            rot_hash: Default::default(),
            collected_fruit: false, 
            current_direction: [0, 0],
            count: 0, 
            count_limit
        };

        snake_struct.key_hash = globals::direction_hash();
        snake_struct.rot_hash = globals::rot_hash(&snake_struct);

        // With this new snake instance, spawn the snake in a random place.
        Self::_random_spawn(&mut snake_struct, gb.screen_width, gb.screen_height);
        return snake_struct;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for i in 0..self.arr.len() {
            if i == 0 {
                let rot_tup = self.rot_hash[&self.current_direction];
                d.draw_texture_ex(&self.head_texture, 
                                  Vector2{x: (self.arr[0][0] * self.head_texture.width()) as f32 + rot_tup.1.0, 
                                                  y: (self.arr[0][1] * self.head_texture.height()) as f32 + rot_tup.1.1}, 
                                  rot_tup.0 as f32, 
                                  1_f32, 
                                  Color::WHITE);
            }
            else {
                d.draw_texture(&self.body_texture, 
                                self.arr[i][0] * self.head_texture.width(), 
                                self.arr[i][1] * self.head_texture.height(), 
                                Color::WHITE);
            }
        }
    }
    
    pub fn update(&mut self, key: &KeyboardKey, screen_width: i32, screen_height: i32) {
        // Guard clause. Exits if the count does not equal the count limit.
        self.count += 1;
        if self.count <= self.count_limit { return ; }
        self.count = 0;

        Self::_check_if_snake_hit_itself(self);
        Self::_prevent_snake_from_leaving_window(self, screen_width, screen_height);
        Self::_move_snake(self, key);

    }

    fn _random_spawn(&mut self, screen_width: i32, screen_height: i32) {
        let random_x: i32 = grv(5, screen_width / self.head_texture.width() - 5);
        let random_y: i32 = grv(5, screen_height / self.head_texture.height() - 5);
        self.arr.push_back([random_x, random_y]);
        for i in 1..self.starting_length {
            self.arr.push_back([self.arr[0][0] + i, self.arr[0][1]]);
        }
    }

    fn _check_if_snake_hit_itself(&mut self) {
        for i in 1..self.arr.len() {
            if self.arr[0] == self.arr[i] {
                self.hit_self = true;
                return ;
            }
        }
    }

    fn _move_snake(&mut self, key: &KeyboardKey) {
        let dir_x;
        let dir_y;
        
        let opposite_direction: [i32; 2];

        if self.key_hash.contains_key(&key) {
            opposite_direction = [-self.key_hash[&key][0], -self.key_hash[&key][1]];

            if self.current_direction != opposite_direction {
                self.current_direction = self.key_hash[&key];
                dir_x = self.current_direction[0];
                dir_y = self.current_direction[1];
            } 
            else {
                dir_x = opposite_direction[0];
                dir_y = opposite_direction[1];
            }
        } 
        else {
            dir_x = self.current_direction[0];
            dir_y = self.current_direction[1];
        }

        if self.current_direction == globals::IDLE { return ; }

        self.arr.push_front([self.arr[0][0] + dir_x, self.arr[0][1] + dir_y]);

        if self.collected_fruit {
            self.score += 1;
            self.collected_fruit = false;
        }
        else {
            self.arr.pop_back();
        }
    }

    fn _prevent_snake_from_leaving_window(&mut self, screen_width: i32, screen_height: i32) {
        let min_x = 0;
        let min_y = 0;
        let max_x = (screen_width / self.head_texture.width()) - 1;
        let max_y = (screen_height / self.head_texture.height()) - 1;

        if self.arr[0][0] < min_x {
            self.arr[0][0] = max_x;
        }
        else if self.arr[0][0] > max_x {
            self.arr[0][0] = 0;
        }
        else if self.arr[0][1] < min_y {
            self.arr[0][1] = max_y;
        } 
        else if self.arr[0][1] > max_y {
            self.arr[0][1] = 0;
        }
    }
}