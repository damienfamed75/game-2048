// standard libraries
use std::thread;
use std::time::Duration;
// internal modules
use crate::grid::{Grid, GRID_WIDTH, GRID_HEIGHT};
// external libraries
use device_query::{DeviceQuery, DeviceState, Keycode};

// How long the update function pauses after drawing.
const UPDATE_PAUSE_MILLIS: u64 = 250;

// Game keeps track of the drawing, controls, and score.
pub struct Game {
    device: DeviceState,
    pub score: i32,
    pub map: Grid,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
            device: DeviceState::new(),
            map: Grid::new(),
        }
	}
	// When update returns an Err(()) it means the game is lost.
    pub fn update(&mut self) -> Result<(), ()> {
        let keys: Vec<Keycode> = self.device.get_keys();
        for key in keys.iter() {
			// Only try to move if it's a valid movement key.
            if is_move_key(key) {
				self.update_movement(key);
				// Try to create a new block.
				// If unsuccessful then the error is propogated.
				self.map.new_rand_block()?;
                self.draw();
                println!("Pressed key: {:?}", key);
                // Sleep on this thread so it doesn't draw a billion times from
                // a single key press.
                thread::sleep(Duration::from_millis(UPDATE_PAUSE_MILLIS));
            }
		}
		Ok(())
	}
	pub fn update_movement(&mut self, dir: &Keycode) {
		// Since the game coordinates are actually set up like this:
		//
		//		0 ⇒ y
		//		⇓
		//		x
		//
		// (down +x) (up -x)
		// (left -y) (right +y)
		//
		// We have to match what the user would expect when pressing
		// left, right, up, and down. Such as this:
		//
		//		y
		//		⇑
		//		0 ⇒ x
		// 
		// That's why the x, and y are flipped around in a confusing manner
		// when calling the functions to move.
		match dir.to_owned() {
			Keycode::Right => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).rev().by_ref(),
					0, 1, // right +y
				);
			},
			Keycode::Left => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					0, -1, // left -y
				);
			},
			Keycode::Down => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).rev().by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					1, 0, // down +x
				);
			},
			Keycode::Up => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					-1, 0, // up -x
				);
			},
			_ => {}, // Omit other key codes.
		}
	}
    pub fn draw(&self) {
		// Clear the terminal and set cursor to first row, first column.
		print!("\x1B[2J\x1B[1;1H");
		// Display score.
		print!("score: {} |", self.score);
		// Display controls.
		println!(" controls: arrow keys(↑ ↓ ← →)");
		// Draw the map.
        println!("{}", self.map);
	}
}

fn is_move_key(key: &Keycode) -> bool {
    (*key == Keycode::Up)
        || (*key == Keycode::Down)
        || (*key == Keycode::Left)
        || (*key == Keycode::Right)
}
