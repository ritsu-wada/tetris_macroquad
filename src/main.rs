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
    x: isize,
    y: isize,
    current_shape: [[u8; 4]; 4],
}

fn can_move(grid: &[[u8; 10]; 20], shape: &[[u8; 4]; 4], x: isize, y: isize) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if shape[i][j] != 0 {
                let gy = y + i as isize - 1;
                let gx = x + j as isize - 1;
                if gy < 0 || gx < 0 || gy > 19 || gx > 9 || grid[gy as usize][gx as usize] != 0 {
                    return false;
                }
            }
        }
    }
    true
}

#[macroquad::main("tetris")]
async fn main() {
    set_window_size(400, 500);
    let mut state = GameState::Title;

    const BLOCK_SIZE: f32 = 20.;
    const SHAPES: [[[u8; 4]; 4]; 7] = [
        // T
        [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        // L
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        // J
        [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        // S
        [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        // Z
        [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        // I
        [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        // O
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    ];

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
                            y: 1,
                            current_shape: SHAPES[macroquad::rand::gen_range(0, 7)],
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
                if is_key_pressed(KeyCode::Left)
                    && controled.x > 0
                    && can_move(
                        &grid,
                        &controled.current_shape,
                        controled.x - 1,
                        controled.y,
                    )
                {
                    controled.x -= 1;
                }
                if is_key_pressed(KeyCode::Right)
                    && can_move(
                        &grid,
                        &controled.current_shape,
                        controled.x + 1,
                        controled.y,
                    )
                {
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
                    if can_move(&grid, &rotated, controled.x, controled.y) {
                        controled.current_shape = rotated;
                    }
                }

                // down calcurate
                if get_time() - *timer > 1. || is_key_pressed(KeyCode::Down) {
                    if {
                        can_move(
                            &grid,
                            &controled.current_shape,
                            controled.x,
                            controled.y + 1,
                        )
                    } {
                        controled.y += 1;
                    } else {
                        //fill block
                        for i in 0..4 {
                            for j in 0..4 {
                                if controled.current_shape[i][j] != 0 {
                                    grid[(controled.y + i as isize - 1) as usize]
                                        [(controled.x + j as isize - 1) as usize] =
                                        controled.current_shape[i][j];
                                }
                            }
                        }
                        //check line is filled
                        let mut level = 19;
                        while level > 0 {
                            if !grid[level].contains(&0) {
                                for target_y in (1..=level).rev() {
                                    grid[target_y] = grid[target_y - 1];
                                }
                                grid[0] = [0; 10];
                            } else {
                                level -= 1;
                            }
                        }
                        if controled.x == 4 && controled.y == 1 && {
                            !can_move(&grid, &controled.current_shape, controled.x, controled.y)
                        } {
                            is_game_over = true;
                        } else {
                            controled.x = 4;
                            controled.y = 1;
                            controled.current_shape = SHAPES[macroquad::rand::gen_range(0, 7)];
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
                                (controled.x as f32 + cx as f32 - 1.0) * BLOCK_SIZE
                                    + (screen_width() / 4.),
                                (controled.y as f32 + cy as f32 - 1.0) * BLOCK_SIZE + 50.,
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
