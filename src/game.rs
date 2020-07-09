use std::thread;
use std::time::Duration;

use crate::grid::Grid;

use rand::Rng;
use device_query::{DeviceQuery, DeviceState, Keycode};

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
    pub fn update(&mut self) {
        let keys: Vec<Keycode> = self.device.get_keys();
        for key in keys.iter() {
            if is_move_key(key) {
				let delta_score = self.map.mov(key);
				self.score += delta_score;
				self.map.new_rand_block();
                self.draw();
                println!("Pressed key: {:?}", key);
                // Sleep on this thread so it doesn't draw a billion times from
                // a single key press.
                thread::sleep(Duration::from_millis(UPDATE_PAUSE_MILLIS));
            }
        }
    }
    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("score: {}", self.score);
        println!("{}", self.map.to_string());
	}
}

fn is_move_key(key: &Keycode) -> bool {
    (*key == Keycode::Up)
        || (*key == Keycode::Down)
        || (*key == Keycode::Left)
        || (*key == Keycode::Right)
}
