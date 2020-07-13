use crate::{
    grid::{Grid, GRID_WIDTH},
    object::{BOX_HEIGHT, BOX_WIDTH},
};
use std::fmt::{self, Display};

const OFFSET: i32 = 6; // offset the drawing from the top of the terminal.

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Grid(arr) = self; // Extract array from grid tuple.
        for x in 0..GRID_WIDTH {
            // Write the formatted object to the string.
            let write_x: i32 = (BOX_HEIGHT * (x + 1) as i32) + OFFSET;
            for (y, obj) in arr[x].iter().enumerate() {
                let write_y: i32 = BOX_WIDTH * ((y + 1) as i32);
                // Set the cursor position to our writing coordinates.
                write!(f, "\x1B[{};{}H", write_x, write_y)?;
                // Draw the object at the writing coordinates.
                write!(f, "{}", obj.write_box(write_x, write_y),)?;
                // If this is the last of this row, then print a new line character.
                if arr[x].len() - 1 == y {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}
