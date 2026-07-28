use macroquad::{miniquad::window::set_window_size, prelude::*};

enum GameState {
    Title,
    Playing { grid: [[u8; 10]; 20] },
    GameOver,
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
                    };
                }
            }
            GameState::Playing { mut grid } => {
                // logic
                grid[19][0] = 1;

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
            }
            GameState::GameOver => {}
        }
        next_frame().await;
    }
}
