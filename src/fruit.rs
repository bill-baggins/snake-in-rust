use crate::raylib_build::GameBuild;
use raylib::prelude::*;
use raylib::core::misc::get_random_value as grv;

pub struct Fruit {
    pub texture: Texture2D,
    pub pos: [i32; 2],
}

impl Fruit {
    pub fn new(path: &str, screen_width: i32, screen_height: i32, scale: f32, gb: &mut GameBuild) -> Fruit {
        let mut texture = Image::load_image(path).expect("Unable to load image 'fruit.png'");

        if scale != 0f32 {
            let new_texture_width = (texture.width() as f32 * scale) as i32;
            let new_texture_height = (texture.height() as f32 * scale) as i32;
            Image::resize(&mut texture, new_texture_width, new_texture_height);
        }
        let texture = gb.rl.load_texture_from_image(&gb.thread, &texture).expect("Unable to convert image 'fruit.png' to a Texture2D");

        let pos: [i32; 2] = [0, 0];
        let mut fruit = Fruit{texture, pos};
        Self::spawn_in_random_place(&mut fruit, screen_width, screen_height);
        return fruit;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let texture_width = self.texture.width();
        let texture_height = self.texture.height();
        d.draw_texture(&mut self.texture, self.pos[0] * texture_width, self.pos[1] * texture_height, Color::WHITE);
    }

    pub fn spawn_in_random_place(&mut self, screen_width: i32, screen_height: i32) {
        let max_x = (screen_width / self.texture.width()) - 5;
        let max_y = (screen_height / self.texture.height()) - 5;

        let x: i32 = grv(5, max_x);
        let y: i32 = grv(5, max_y);
        self.pos = [x, y];
    }
}