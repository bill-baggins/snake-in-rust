pub mod raylib_build;
// pub mod event_manager;
pub mod game;
pub mod snake;
pub mod grid;
pub mod fruit;
pub mod globals;

use raylib_build::GameBuild;

fn main() {
    let mut gb = GameBuild::init_handler_and_thread(1200, 800, "Test", false, 60);
    game::game_loop(&mut gb);
}