

use crate::raylib_build::GameBuild;

use raylib::prelude::*;

pub struct Grid {
    pub image: Texture2D
}

impl Grid {
    pub fn new(path: &str, gb: &mut GameBuild, scale: f32) -> Grid {
        let mut image = Image::load_image(path).expect("Unable to load image empty_square");

        if scale != 1f32 {
            let im_width = image.width();
            let im_height = image.height();
            Image::resize(&mut image, (im_width as f32 * scale).round() as i32, (im_height as f32 * scale).round() as i32);
        }

        let image = gb.rl.load_texture_from_image(&gb.thread, &image)
                .expect("Unable to load background texture");
        Grid{image}
    }

    pub fn draw(&mut self, grid_x: i32, grid_y: i32, d: &mut RaylibDrawHandle) {
        let image_width = self.image.width();
        let image_height=  self.image.height();

        for x in 0..grid_x {
            for y in 0..grid_y {
                d.draw_texture(&mut self.image, x * image_width, y * image_height, Color::WHITE);
            } 
        }
    }
}