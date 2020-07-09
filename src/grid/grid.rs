// standard libraries
use std::iter::Iterator;
// internal modules
use crate::object::Object;
// external libraries
use rand::Rng;

pub const GRID_WIDTH: usize			= 4; // width
pub const GRID_HEIGHT: usize		= 4; // height
const BLOCK_DEFAULT_NUMBER: i16		= 2; // random block starting value
const MAX_RETRY_RAND_BLOCK: i32		= 64;

// vertical bars │ │ │
// separator ──────
//
// ┌──────┬──────┐
//
// ├──────┼──────┤
//
// └──────┴──────┘

// Grid is a 4x4 2 dimensional array each containing a block.
pub struct Grid(pub [[Object; GRID_HEIGHT]; GRID_WIDTH]);

impl Grid {
    pub fn new() -> Self {
        let mut grid = Self([[Object::default(); GRID_HEIGHT]; GRID_WIDTH]);
        // Generate two random variables to spawn.
		grid.new_rand_block().unwrap();
		grid.new_rand_block().unwrap();
		// Return the grid with the new two random blocks.
        grid
	}
	// creates a new random block with a default value on a random spot of the
	// grid.
	//
	// After meeting the max retry count to find an empty spot on the grid,
	// an error is thrown to indicate that the game is lost.
	pub fn new_rand_block(&mut self) -> Result<(), ()> {
		let mut tries = 0; // number of retries to place a random block.
        loop {
            let mut rng = rand::thread_rng();
            let (x, y) = (rng.gen_range(0, GRID_WIDTH), rng.gen_range(0, GRID_HEIGHT));
            // If this coordinate is empty then add a new block there.
            if let Object::Empty = self.0[x][y] {
                self.0[x][y] = Object::Block(BLOCK_DEFAULT_NUMBER);
                return Ok(());
			}
			tries += 1;
			if tries >= MAX_RETRY_RAND_BLOCK {
				return Err(());
			}
        }
	}
	// mov_dir moves in a direction specified.
	// -1 or 1 to move in that direction whether it be x or y.
	// 0 means no movement on this axis.
	//
	// Pass in the iterator for the x and y axis.
	// Iterator is passed in because movement on an axis may affect what direction
	// you want to check the blocks in.
	pub fn mov_dir<X: Iterator+Clone, Y: Iterator+Clone>(
		&mut self,
		x_iter: &mut X,
		y_iter: &mut Y,
		dx: i8, dy: i8,
	) -> i32
	where 
		X: Iterator<Item = usize>,
		Y: Iterator<Item = usize>
	{
		// How much the score should change by.
		let mut delta_score: i32 = 0;
		for x in x_iter.into_iter() {
			// We must clone the iterator because the reference is used up after
			// the first loop.
			for y in y_iter.clone().into_iter() {
				if let Object::Block(number) = self.0[x][y] {
					let (mut ox,mut oy): (i8, i8) = (x as i8, y as i8);
					let mut onum = number; // var for original number.
					loop {
						let (nx, ny, nnum) = self.mov_delta(onum, ox, oy, dx, dy);
						if (nx, ny) == (ox, oy) { break; }
						if onum != nnum { delta_score += nnum as i32; }
						// Originals equal the new values.
						ox = nx;
						oy = ny;
						onum = nnum;
					}
				}
			}
		}
		delta_score
	}
	// obj_at returns an object if one exists at the provided [x,y] coordinates.
    fn obj_at(&mut self, x: usize, y: usize) -> Option<Object> {
		// If the x and y coordinates are out of bounds of the grid.
        if (x >= GRID_WIDTH) || (y >= GRID_HEIGHT) {
            None
        } else {
			Some(self.0[x][y])
		}
	}
    fn mov_delta(&mut self, number: i16, x: i8, y: i8, dx: i8, dy: i8) -> (i8, i8, i16) {
		if dx != 0 {
			if let Some(obj) = self.obj_at((x+dx) as usize, y as usize) {
				match obj {
					Object::Block(other_number) => {
						if number == other_number {
							self.0[(x+dx) as usize][y as usize] = Object::Block(other_number + number);
							self.0[x as usize][y as usize] = Object::Empty;
							return (x+dx, y+dy, other_number+number);
						}
					}
					Object::Empty => {
						self.0[(x+dx) as usize][y as usize] = Object::Block(number);
						self.0[x as usize][y as usize] = Object::Empty;
						return (x+dx, y, number);
					}
				}
			}
		} else if dy != 0 {
			if let Some(obj) = self.obj_at(x as usize, (y+dy) as usize) {
				match obj {
					Object::Block(other_number) => {
						if number == other_number {
							self.0[x as usize][(y+dy) as usize] = Object::Block(other_number + number);
							self.0[x as usize][y as usize] = Object::Empty;
							return (x, y+dy, other_number+number);
						}
					}
					Object::Empty => {
						self.0[x as usize][(y+dy) as usize] = Object::Block(number);
						self.0[x as usize][y as usize] = Object::Empty;
						return (x, y+dy, number);
					}
				}
			}
		}
        (x, y, number)
	}
}
