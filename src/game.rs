// standard libraries
use std::thread;
use std::time::Duration;
// internal modules
use crate::grid::{Grid, GRID_WIDTH, GRID_HEIGHT};
// external libraries
use device_query::{DeviceQuery, DeviceState, Keycode};

// How long the update function pauses after drawing.
const UPDATE_PAUSE_MILLIS: u64 = 500;

pub struct Game {
    pub score: i32,
    device: DeviceState,
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
    pub fn update(&mut self) -> Result<(), ()> {
        let keys: Vec<Keycode> = self.device.get_keys();
        for key in keys.iter() {
			// Only try to move if it's a valid movement key.
            if is_move_key(key) {
				self.update_movement(key);
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
		match *dir {
			Keycode::Right => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).rev().by_ref(),
					0, 1,
				);
			},
			Keycode::Left => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					0, -1,
				);
			},
			Keycode::Up => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					-1, 0,
				);
			},
			Keycode::Down => {
				self.score += self.map.mov_dir(
					(0..GRID_WIDTH).rev().by_ref(),
					(0..GRID_HEIGHT).by_ref(),
					1, 0,
				);
			},
			_ => {},
		}
	}
    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("score: {} | ", self.score);
        println!("{}", self.map);
	}
}

fn is_move_key(key: &Keycode) -> bool {
    (*key == Keycode::Up)
        || (*key == Keycode::Down)
        || (*key == Keycode::Left)
        || (*key == Keycode::Right)
}
