mod game;
pub mod grid;
pub mod object;

use game::Game;

fn main() {
    let mut g = Game::new();
    // Draw the first frame.
    g.draw();
    // Game loop.
    loop {
        if let Err(()) = g.update() {
            print!("You LOST");
            return;
        }
    }
}
