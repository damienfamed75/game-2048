use crate::object::Object;

use device_query::Keycode;
use rand::Rng;

const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;

// const BARS: [(&str, &str, &str); 3] = [("┌", "┬", "┐"), ("├", "┼", "┤"), ("└", "┴", "┘")];
// const SEPERATOR: &str = "──────";

const BOX_WIDTH: usize = 8;

// Grid is a 4x4 2 dimensional array each containing a block.
// type Grid = [[Object; 4]; 4];
pub struct Grid([[Object; GRID_HEIGHT]; GRID_WIDTH]);

impl Grid {
    pub fn new() -> Self {
        let mut grid = Self([[Object::default(); GRID_HEIGHT]; GRID_WIDTH]);
        // Generate two random variables to spawn.
        let mut j = 0;
        while j < 2 {
            let mut rng = rand::thread_rng();
            let (x, y) = (rng.gen_range(0, GRID_WIDTH), rng.gen_range(0, GRID_HEIGHT));
            // If this coordinate is empty then add a new block there.
            if let Object::Empty = grid.0[x][y] {
                grid.0[x][y] = Object::Block(2);
                j += 1;
            }
        }

        grid
    }
    pub fn to_string(&self) -> String {
        let arr = self.0;
        let mut response = String::new();

        response.push_str(&format!(
            "┌{0:─>width$}┬{0:─>width$}┬{0:─>width$}┬{0:─>width$}┐",
            "",
            width = BOX_WIDTH
        ));

        for (x, _) in arr.iter().enumerate() {
            response.push_str("\n│");
            response.push_str(&format!("{: >width$}│", "", width = BOX_WIDTH).repeat(arr[x].len()));
            response.push_str("\n│");

            // Write the formatted object to the string.
            for (_, obj) in arr[x].iter().enumerate() {
                response.push_str(&obj.fmt_width(BOX_WIDTH));
            }

            response.push_str("\n│");
            response.push_str(&format!("{: >width$}│", "", width = BOX_WIDTH).repeat(arr[x].len()));
            // If this isn't the last line, then draw connectors between all of
            // the boxes.
            if x != arr.len() - 1 {
                response.push_str(&format!("\n├{:─>width$}┼", "─", width = BOX_WIDTH));
                response.push_str(
                    &format!("{:─>width$}┼", "─", width = BOX_WIDTH).repeat(arr[x].len() - 2),
                );
                response.push_str(&format!("{:─>width$}┤", "─", width = BOX_WIDTH));
            }
        }

        response.push_str(&format!(
            "\n└{0:─>width$}┴{0:─>width$}┴{0:─>width$}┴{0:─>width$}┘",
            "",
            width = BOX_WIDTH
        ));

        response
    }
    pub fn is_block(&mut self, x: usize, y: usize) -> bool {
        self.0[x][y] != Object::Empty
    }
    pub fn block_at(&mut self, x: usize, y: usize, dx: usize, dy: usize) -> Object {
        if !is_valid_x(x + dx) || !is_valid_y(y + dy) {
            return Object::Empty;
        }
        self.0[x + dx][y + dy]
    }
    pub fn obj_at(&mut self, x: usize, y: usize) -> Option<Object> {
        if !is_valid_x(x) || !is_valid_y(y) {
            return None;
        }
        Some(self.0[x][y])
    }
    pub fn mov(&mut self, number: i16, x: i8, y: i8, dx: i8, dy: i8) {
        for xx in x..(x + dx) {
            if let Some(Object::Block(other_number)) = self.obj_at(xx as usize, y as usize) {
                if number == other_number {
                    self.0[xx as usize][y as usize] = Object::Block(other_number + number);
                    self.0[x as usize][y as usize] = Object::Empty;
                }
            }
        }
        for _yy in y..(y + dy) {}
    }
    pub fn mov_direction(&mut self, _dir: &Keycode) {
        // let arr = &mut self.0;
        // for (x, row) in arr.iter().enumerate() {
        //     for (y, col) in row.iter().enumerate() {
        //         if let Object::Block(number) = col {
        //             match *dir {
        //                 Keycode::Up => self.mov(*number, x as i8, y as i8, 0, GRID_HEIGHT as i8),
        //                 Keycode::Down => {
        //                     self.mov(*number, x as i8, y as i8, 0, GRID_HEIGHT as i8 * -1)
        //                 }
        //                 Keycode::Left => self.mov(*number, x as i8, y as i8, GRID_WIDTH as i8, 0),
        //                 Keycode::Right => {
        //                     self.mov(*number, x as i8, y as i8, GRID_WIDTH as i8 * -1, 0)
        //                 }
        //                 _ => self.mov(0, x as i8, y as i8, 0, 0),
        //             }
        //         }
        //     }
        // }
    }
}

fn is_valid_x(x: usize) -> bool {
    x < GRID_WIDTH
}
fn is_valid_y(y: usize) -> bool {
    y < GRID_HEIGHT
}
