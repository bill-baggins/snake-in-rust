use crate::raylib_build::GameBuild;
use crate::grid::Grid;
use crate::fruit::Fruit;
use crate::snake::Snake;
use crate::globals::{GameState};
use raylib::prelude::*;
use std::env;
use std::path::PathBuf;

// TODO: Add an enemy snake that chases the player around.

pub fn game_loop(gb: &mut GameBuild) {
    let screen_width = gb.rl.get_screen_width();
    let screen_height = gb.rl.get_screen_height();
    let scale = 2.5;
    let count_limit = 7;

    let base_dir = option_env!("CARGO_MANIFEST_DIR").map_or_else(|| {
        let exe_path = env::current_exe().expect("Failed to get exe path");
        exe_path.parent().expect("Failed to get exe dir").to_path_buf()
    }, |crate_dir| {
        PathBuf::from(crate_dir)
    });

    let resources_dir = base_dir.join("resources");

    // Initialization of objects
    let grid_path = resources_dir.join("empty_square.png").display().to_string();
    let fruit_path = resources_dir.join("fruit.png").display().to_string();
    let snake_head_path = resources_dir.join("snake_head.png").display().to_string();
    let snake_body_path = resources_dir.join("snake.png").display().to_string();

    let grid_path = grid_path.as_str();
    let fruit_path = fruit_path.as_str();
    let snake_head_path = snake_head_path.as_str();
    let snake_body_path = snake_body_path.as_str();

    let mut game_state = GameState::Paused;

    let mut grid = Grid::new(grid_path, gb, scale);
    let grid_x = screen_width / grid.image.width();
    let grid_y = screen_height / grid.image.height();

    let mut snake = Snake::new(snake_head_path, snake_body_path, 3, scale, count_limit, gb);
    let mut fruit = Fruit::new(fruit_path, &snake.arr[0], screen_width, screen_height, scale, gb);
    let mut current_key = KeyboardKey::KEY_ZERO;

    let pause_text = "Press the arrow keys to begin!";
    let game_over_text = "Game over! Press the R key to restart.";
    let snake_score_text = |s: &Snake| format!("Score: {}", s.score);

    // Main game loop. Will exit if the user presses the escape key.
    while !gb.rl.window_should_close() {
        // Get key input from the user. Defaults to the ZERO key otherwise.
        // In other languages, I would have just kept it null and do a null check. However,
        // I cannot do that here. So, my default key is set to ZERO.
        let key = gb.rl.get_key_pressed().unwrap_or(KeyboardKey::KEY_ZERO);

        // This statement checks to see if the snake's key-hash contains the pressed key and whether or not
        // the snake has hit itself. It alsos starts the game if the current game_state is paused.
        if snake.key_hash.contains_key(&key) && current_key != key && game_state != GameState::Over {
            if game_state == GameState::Paused { game_state = GameState::Running; }
            current_key = key;
        }
        
        // Update the game depending on the game's state.
        // When it is running, the game will update the snake and check to see if the snake has hit itself.
        // If the snake has hit itself, the game is over. At this point, the game gives the player an option
        // to restart.
        match game_state {
            GameState::Running => { 
                Snake::update(&mut snake, &current_key, &mut fruit, screen_width, screen_height);

                if snake.hit_self {
                    game_state = GameState::Over;
                }

            },
            GameState::Over => {
                // Give the player the option to restart the game upon making the snake hit itself.
                if gb.rl.is_key_pressed(KeyboardKey::KEY_R) {
                    game_state = GameState::Paused;
                    fruit = Fruit::new(fruit_path, &snake.arr[0], screen_width, screen_height, scale, gb);
                    snake = Snake::new(snake_head_path, snake_body_path, 3, scale, count_limit, gb);
                    current_key = KeyboardKey::KEY_ZERO;
                }
            },
            _ => {}
        }
        
        // Create an instance of RaylibDrawHandle. This is where all the game's drawing is done.
        let mut d = gb.rl.begin_drawing(&gb.thread);

        d.clear_background(Color::RAYWHITE);
        
        // Draw to the screen depending on the current game state.
        match game_state {

            // Draws the grid, fruit, snake. The game's starting text is drawn in the middle of the screen 
            // which reads "Press the arrow keys to begin!"
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

            // Draws the grid, fruit, snake. The player's score is drawn at the top right corner of the
            // screen.
            GameState::Running => {
                let score = snake_score_text(&snake);
                Grid::draw(&mut grid, grid_x, grid_y, &mut d);
                Fruit::draw(&mut fruit, &mut d);
                Snake::draw(&mut snake, &mut d);
                d.draw_text(score.as_str(), 
                            screen_width - (score.len() as i32 + (grid.image.width() * 5)), 
                            0, 
                            grid.image.width(), 
                            Color::YELLOW);
            },

            // Draws the grid, fruit, and snake. The game over text is drawn in the middle of the screen,
            // which reads "Game over! Press the R key to restart."
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
    }
}

