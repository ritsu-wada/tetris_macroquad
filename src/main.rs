use macroquad::prelude::*;

enum GameState {
    Title,
    InGame { grid: [[u8; 10]; 20] },
    GameOver,
}

#[macroquad::main("tetris")]
async fn main() {
    let mut state = GameState::Title;
    loop {
        clear_background(BLACK);
        match state {
            GameState::Title => {
                draw_text("TETRIS", 100., 100., 40., WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::InGame {
                        grid: [[0; 10]; 20],
                    };
                }
            }
            GameState::InGame { grid } => {
                // logic

                // draw
            }
            GameState::GameOver => {}
        }
        next_frame().await;
    }
}
