use crate::raylib_build::GameBuild;
use crate::grid::Grid;
use crate::fruit::Fruit;
use crate::snake::Snake;
use crate::globals::{GameState};
use raylib::prelude::*;


pub fn game_loop(gb: &mut GameBuild) {
    let screen_width = gb.rl.get_screen_width();
    let screen_height = gb.rl.get_screen_height();
    let scale = 2.5;
    let count_limit = 7;

    // Initialization of objects.
    let grid_path = "src/resources/empty_square.png";
    let fruit_path = "src/resources/fruit.png";
    let snake_head_path = "src/resources/snake_head.png";
    let snake_body_path = "src/resources/snake.png";
    let mut game_state = GameState::Paused;

    let mut grid = Grid::new(grid_path, gb, scale);
    let grid_x = screen_width / grid.image.width();
    let grid_y = screen_height / grid.image.height();

    let mut fruit = Fruit::new(fruit_path, screen_width, screen_height, scale, gb);
    let mut snake = Snake::new(snake_head_path, snake_body_path, 3, scale, count_limit, gb);
    let mut current_key = KeyboardKey::KEY_ZERO;

    let pause_text = "Press the arrow keys to begin!";
    let game_over_text = "Game over! Press the R key to restart.";
    let snake_score_text = |s: &Snake| format!("Score: {}", s.score);

    while !gb.rl.window_should_close() {
        let key = gb.rl.get_key_pressed().unwrap_or(KeyboardKey::KEY_ZERO);

        if snake.key_hash.contains_key(&key) && current_key != key && game_state != GameState::Over {
            if game_state == GameState::Paused { game_state = GameState::Running; }
            current_key = key;
        }

        match game_state {
            GameState::Running => { 
                Snake::update(&mut snake, &current_key, &mut fruit, screen_width, screen_height);

                if snake.hit_self {
                    game_state = GameState::Over;
                }

            },
            GameState::Over => {
                if gb.rl.is_key_pressed(KeyboardKey::KEY_R) {
                    game_state = GameState::Paused;
                    fruit = Fruit::new(fruit_path, screen_width, screen_height, scale, gb);
                    snake = Snake::new(snake_head_path, snake_body_path, 3, scale, count_limit, gb);
                    current_key = KeyboardKey::KEY_ZERO;
                }
            },
            _ => {}
        }

        let mut d = gb.rl.begin_drawing(&gb.thread);
        d.clear_background(Color::RAYWHITE);

        match game_state {
            GameState::Paused => {
                Grid::draw(&mut grid, grid_x, grid_y, &mut d);
                Fruit::draw(&mut fruit, &mut d);
                Snake::draw(&mut snake, &mut d);
                d.draw_text(pause_text,
                    (screen_width / 2) - (pause_text.len() * grid.image.width() as usize / 4) as i32 ,
                    screen_height / 2,
                    grid.image.width(),
                    Color::YELLOW);
            },
            GameState::Running => {
                let score = snake_score_text(&snake);
                Grid::draw(&mut grid, grid_x, grid_y, &mut d);
                Fruit::draw(&mut fruit, &mut d);
                Snake::draw(&mut snake, &mut d);
                d.draw_text(score.as_str(), 
                            screen_width - (pause_text.len() as i32 + (grid.image.width() * 4)), 
                            0, 
                            grid.image.width(), 
                            Color::YELLOW);
            },
            GameState::Over => {
                Grid::draw(&mut grid, grid_x, grid_y, &mut d);
                Fruit::draw(&mut fruit, &mut d);
                Snake::draw(&mut snake, &mut d);
                d.draw_text(game_over_text,
                    (screen_width / 2) - (game_over_text.len() * grid.image.width() as usize / 4) as i32,
                    screen_height / 2, 
                    grid.image.width(),
                    Color::YELLOW);
            }
        }

        d.draw_fps(0, 0);
        
    }
}

