use raylib::prelude::*;

pub struct GameBuild {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub screen_width: i32,
    pub screen_height: i32
}

impl GameBuild {
    pub fn init_handler_and_thread(width: i32, height: i32, title: &str, fullscreen: bool, fps: u32) -> GameBuild {
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        
        let mut gb = GameBuild{rl, thread, screen_width, screen_height};
        if fullscreen { gb.rl.toggle_fullscreen(); }
        gb.rl.set_target_fps(fps);
        return gb;
    }
}