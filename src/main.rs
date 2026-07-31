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
    current_shape: [[u8; 4]; 4],
}

#[macroquad::main("tetris")]
async fn main() {
    set_window_size(400, 500);
    let mut state = GameState::Title;
    const BLOCK_SIZE: f32 = 20.;
    //     const SHAPES = [
    //     // T
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     //
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    //     [0,0,0,0],
    // ]

    loop {
        clear_background(BLACK);
        match state {
            GameState::Title => {
                draw_text("TETRIS", 100., 100., 40., WHITE);
                draw_text("press space", 100., 200., 30., WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing {
                        grid: [[0; 10]; 20],
                        controled: Controled {
                            x: 4,
                            y: 0,
                            current_shape: [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
                        },
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
                let mut is_game_over = false;
                // logic
                if is_key_pressed(KeyCode::Left) && controled.x > 0 && {
                    let mut can_move = true;
                    for i in 0..4 {
                        for j in 0..4 {
                            if controled.current_shape[i][j] != 0 {
                                let next_x = controled.x + j - 1;
                                let next_y = controled.y + i;
                                if next_y > 19 || next_x > 9 || grid[next_y][next_x] != 0 {
                                    can_move = false;
                                }
                            }
                        }
                    }
                    can_move
                } {
                    controled.x -= 1;
                }
                if is_key_pressed(KeyCode::Right) && {
                    let mut can_move = true;
                    for i in 0..4 {
                        for j in 0..4 {
                            if controled.current_shape[i][j] != 0 {
                                let next_x = controled.x + j + 1;
                                let next_y = controled.y + i;
                                if next_y > 19 || next_x > 9 || grid[next_y][next_x] != 0 {
                                    can_move = false;
                                }
                            }
                        }
                    }
                    can_move
                } {
                    controled.x += 1;
                }

                // rotate
                if is_key_pressed(KeyCode::Space) {
                    let mut rotated = [[0; 4]; 4];
                    for y in 0..4 {
                        for x in 0..4 {
                            rotated[x][3 - y] = controled.current_shape[y][x];
                        }
                    }
                    if {
                        let mut can_move = true;
                        for i in 0..4 {
                            for j in 0..4 {
                                if rotated[i][j] != 0 {
                                    let next_x = controled.x + j;
                                    let next_y = controled.y + i;
                                    if next_y > 19 || next_x > 9 || grid[next_y][next_x] != 0 {
                                        can_move = false;
                                    }
                                }
                            }
                        }
                        can_move
                    } {
                        controled.current_shape = rotated;
                    }
                }

                // down calcurate
                if get_time() - *timer > 0.3 || is_key_pressed(KeyCode::Down) {
                    if {
                        let mut can_move = true;
                        for i in 0..4 {
                            for j in 0..4 {
                                if controled.current_shape[i][j] != 0 {
                                    let next_x = controled.x + j;
                                    let next_y = controled.y + i + 1;
                                    if next_y > 19 || grid[next_y][next_x] != 0 {
                                        can_move = false;
                                    }
                                }
                            }
                        }
                        can_move
                    } {
                        controled.y += 1;
                    } else {
                        //fill block
                        for i in 0..4 {
                            for j in 0..4 {
                                if controled.current_shape[i][j] != 0 {
                                    grid[controled.y + i][controled.x + j] =
                                        controled.current_shape[i][j];
                                }
                            }
                        }
                        //check line is filled
                        for y in (0..20).rev() {
                            if !grid[y].contains(&0) {
                                for target_y in (1..=y).rev() {
                                    grid[target_y] = grid[target_y - 1];
                                }
                            }
                        }
                        if controled.x == 4 && controled.y == 0 && {
                            let mut can_move = false;
                            for i in 0..4 {
                                for j in 0..4 {
                                    if controled.current_shape[i][j] != 0 {
                                        let next_x = controled.x + j;
                                        let next_y = controled.y + i;
                                        if next_y > 19 || grid[next_y][next_x] != 0 {
                                            can_move = true;
                                        }
                                    }
                                }
                            }
                            can_move
                        } {
                            is_game_over = true;
                        } else {
                            controled.x = 4;
                            controled.y = 0;
                        }
                    }
                    *timer = get_time();
                }

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
                //drow controled mino
                for cy in 0..4 {
                    for cx in 0..4 {
                        if controled.current_shape[cy][cx] != 0 {
                            draw_rectangle(
                                (controled.x + cx) as f32 * BLOCK_SIZE + (screen_width() / 4.),
                                (controled.y + cy) as f32 * BLOCK_SIZE + 50.,
                                BLOCK_SIZE,
                                BLOCK_SIZE,
                                WHITE,
                            );
                        }
                    }
                }
                if is_game_over {
                    state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                draw_text("Game Over", 100., 100., 40., WHITE);
                draw_text("press space", 100., 200., 30., WHITE);
                if is_key_down(KeyCode::Space) {
                    state = GameState::Title;
                }
            }
        };
        next_frame().await;
    }
}
