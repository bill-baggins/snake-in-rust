use crate::raylib_build::GameBuild;
use raylib::prelude::*;
use raylib::core::misc::get_random_value as grv;

pub struct Fruit {
    pub texture: Texture2D,
    pub pos: [i32; 2],
}

impl Fruit {
    pub fn new(path: &str, scale: f32, gb: &mut GameBuild) -> Fruit {
        let mut texture = Image::load_image(path).expect("Unable to load image 'fruit.png'");

        if scale != 0f32 {
            let new_texture_width = (texture.width() as f32 * scale) as i32;
            let new_texture_height = (texture.height() as f32 * scale) as i32;
            Image::resize(&mut texture, new_texture_width, new_texture_height);
        }
        let texture = gb.rl.load_texture_from_image(&gb.thread, &texture).expect("Unable to convert image 'fruit.png' to a Texture2D");

        let max_x = (gb.rl.get_screen_width() / texture.width()) - 5;
        let max_y = (gb.rl.get_screen_height() / texture.height()) - 5;

        let x = grv(5, max_x);
        let y = grv(5, max_y);

        let pos: [i32; 2] = [x, y];
        Fruit{texture, pos}
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let texture_width = self.texture.width();
        let texture_height = self.texture.height();
        d.draw_texture(&mut self.texture, self.pos[0] * texture_width, self.pos[1] * texture_height, Color::WHITE);
    }
}