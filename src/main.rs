use macroquad::{miniquad::window::set_window_size, prelude::*};

enum GameState {
    Title,
    Playing {
        grid: [[u8; 10]; 20],
        controled: Controled,
        timer: f64,
    },
    GameOver,
}

struct Controled {
    x: usize,
    y: usize,
}

#[macroquad::main("tetris")]
async fn main() {
    set_window_size(400, 500);
    let mut state = GameState::Title;
    const BLOCK_SIZE: f32 = 20.;

    loop {
        clear_background(BLACK);
        match state {
            GameState::Title => {
                draw_text("TETRIS", 100., 100., 40., WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing {
                        grid: [[0; 10]; 20],
                        controled: Controled { x: 4, y: 0 },
                        timer: get_time(),
                    }
                }
            }
            // ref matchはstateから所有権をもぎ取るためループ終わりに破棄されてしまう. match &stateと書いてもいい
            // &は式として使う, refは指示らしい
            GameState::Playing {
                ref mut grid,
                ref mut controled,
                ref mut timer,
            } => {
                // logic
                if is_key_pressed(KeyCode::Left)
                    && controled.x > 0
                    && grid[controled.y][controled.x - 1] != 1
                {
                    controled.x -= 1;
                }
                if is_key_pressed(KeyCode::Right)
                    && controled.x < 9
                    && grid[controled.y][controled.x + 1] != 1
                {
                    controled.x += 1;
                }

                // down calcurate
                if get_time() - *timer > 0.3 || is_key_pressed(KeyCode::Down) {
                    if controled.y < 19 && grid[controled.y + 1][controled.x] != 1 {
                        controled.y += 1;
                    } else {
                        grid[controled.y][controled.x] = 1;
                        //check line is filled
                        for y in (0..20).rev() {
                            if !grid[y].contains(&0) {
                                for target_y in (1..=y).rev() {
                                    grid[target_y] = grid[target_y - 1];
                                }
                            }
                        }
                        if grid[0][4] == 1 {
                        } else {
                            controled.x = 4;
                            controled.y = 0;
                        }
                    }
                    *timer = get_time();
                }

                //tmp
                grid[19][0] = 1;
                grid[19][1] = 1;

                // draw
                for y in 0..grid.len() {
                    for x in 0..grid[y].len() {
                        match grid[y][x] {
                            1 => {
                                draw_rectangle(
                                    x as f32 * BLOCK_SIZE + (screen_width() / 4.),
                                    y as f32 * BLOCK_SIZE + 50.,
                                    BLOCK_SIZE,
                                    BLOCK_SIZE,
                                    WHITE,
                                );
                            }
                            _ => {
                                draw_rectangle_lines(
                                    x as f32 * BLOCK_SIZE + (screen_width() / 4.),
                                    y as f32 * BLOCK_SIZE + 50.,
                                    BLOCK_SIZE,
                                    BLOCK_SIZE,
                                    2.,
                                    WHITE,
                                );
                            }
                        }
                    }
                }
                draw_rectangle(
                    controled.x as f32 * BLOCK_SIZE + (screen_width() / 4.),
                    controled.y as f32 * BLOCK_SIZE + 50.,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    WHITE,
                );
            }
            GameState::GameOver => (),
        };
        next_frame().await;
    }
}
