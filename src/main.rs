mod game;
pub mod grid;
pub mod object;

use game::Game;

fn main() {
    let g = Game::new();
    // Draw the first frame.
    g.draw();
    // Game loop.
    loop {
        g.update();
    }
}
