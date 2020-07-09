use crate::object::Object;
use rand::Rng;
use std::iter::Iterator;

pub const GRID_WIDTH: usize = 4; // width
pub const GRID_HEIGHT: usize = 4; // height
const MAX_RETRY_RAND_BLOCK: i32 = 64;

// Grid is a structure of a tuple containing a 2-dimensional array of objects.
pub struct Grid(pub(crate) [[Object; GRID_HEIGHT]; GRID_WIDTH]);

impl Grid {
    pub fn new() -> Self {
        let mut grid = Self([[Object::default(); GRID_HEIGHT]; GRID_WIDTH]);
        // Generate two random variables to spawn.
        // We unwrap the errors because these calls should never error.
        grid.place_block(2) // place a block of value 2
            .and(grid.place_block(2)) // and place another block of value 2
			.expect("grid has no empty spaces");
        // Return the grid with the new two random blocks.
        grid
    }
    // new_rand_block creates a block with a 1/9 probability of being 4
    // on a random spot of the grid.
    //
    // After meeting the max retry count to find an empty spot on the grid,
    // an error is thrown to indicate that the game is lost.
    pub fn new_rand_block(&mut self) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        // Take the probability of 1/9 which is 10%
        // if the random number generated meets the ratio, then the block
        // will be 4, otherwise default to 2.
        let block_number: i16 = if rng.gen_ratio(1, 9) { 4 } else { 2 };
        self.place_block(block_number) // try to place block.
    }
    // places a block with the given value on a random spot of the grid.
    //
    // After meeting the max retry count to find an empty spot on the grid,
    // an error is thrown to indicate that the game is lost.
    pub fn place_block(&mut self, num: i16) -> Result<(), ()> {
        let mut tries = 0; // number of retries to place a random block.
        loop {
            let mut rng = rand::thread_rng();
            let (x, y) = (rng.gen_range(0, GRID_WIDTH), rng.gen_range(0, GRID_HEIGHT));
            // If this coordinate is empty then add a new block there.
            if let Object::Empty = self.0[x][y] {
                self.0[x][y] = Object::Block(num);
                return Ok(());
            }
            // Add to the retry counter.
            tries += 1;
            if tries >= MAX_RETRY_RAND_BLOCK {
                return Err(()); // No empty spots on the map found.
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
        dx: i8,
        dy: i8,
    ) -> (i32, bool)
    where
        // iterators must be of indices (usize)
        X: Iterator<Item=usize>,
        Y: Iterator<Item=usize>, {
        // How much the score should change by.
        let mut delta_score: i32 = 0;
        let mut should_spawn_new_block = false;
        for x in x_iter.into_iter() {
            // We must clone the iterator because the reference is used up after
            // the first loop.
            for y in y_iter.clone().into_iter() {
                if let Object::Block(number) = self.0[x][y] {
                    // Store copies of the original x, and original y
                    let (mut ox, mut oy) = (x as i8, y as i8);
                    let mut onum = number; // var for original number.
                    loop {
                        let (nx, ny, nnum) = self.mov_delta(onum, ox, oy, dx, dy);
                        // If the block's position didn't change then break.
                        if (nx, ny) == (ox, oy) {
                            break;
                        } else {
                            // A block moved so a new block should be spawned.
                            should_spawn_new_block = true;
                        }
                        // If the block's number changed, then add to the score
                        // and stop movement for this block by breaking.
                        if onum != nnum {
                            delta_score += nnum as i32;
                            break;
                        }
                        // Set the originals equal the new values.
                        ox = nx;
                        oy = ny;
                        onum = nnum;
                    }
                }
            }
        }
        (delta_score, should_spawn_new_block)
    }
    // mov_delta moves the selected block by the delta x and y.
    // If the new position is another block of the same number, they are merged.
    // this function returns (new_x, new_y, new_number)
    fn mov_delta(&mut self, number: i16, x: i8, y: i8, dx: i8, dy: i8) -> (i8, i8, i16) {
        if let Some(obj) = self.obj_at((x + dx) as usize, (y + dy) as usize) {
            match obj {
                Object::Block(other_number) => {
                    // If this neighboring block's value is equal to this.
                    if number == other_number {
                        // Set the neighboring block's value to a new number.
                        self.0[(x + dx) as usize][(y + dy) as usize] =
                            Object::Block(other_number + number);
                        // Set current block's position to empty.
                        self.0[x as usize][y as usize] = Object::Empty;
                        return (x + dx, y + dy, other_number + number);
                    }
                }
                Object::Empty => {
                    // If this neighboring object is empty then move.
                    self.0[(x + dx) as usize][(y + dy) as usize] = Object::Block(number);
                    // Set current block's position to empty.
                    self.0[x as usize][y as usize] = Object::Empty;
                    return (x + dx, y + dy, number);
                }
            }
        }
        // No movement, return what was given.
        (x, y, number)
    }
    // obj_at returns an object if one exists at the provided [x,y] coordinates.
    fn obj_at(&self, x: usize, y: usize) -> Option<Object> {
        // If the x and y coordinates are out of bounds of the grid.
        if (x >= GRID_WIDTH) || (y >= GRID_HEIGHT) {
            None // OOB, return nothing.
        } else {
            Some(self.0[x][y]) // Return object at the coordinates.
        }
    }
}
