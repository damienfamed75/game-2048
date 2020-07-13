use ansi_term::Color;

// Object is used for every part of the grid.
// An object can either contain a number block or it can be empty.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Block(i16), // a block with a number value.
    Empty,
}

// Default returns empty, this is used for initialization.
impl Default for Object {
    fn default() -> Self { Object::Empty }
}

const BOX_INNER_WIDTH: usize = 8;
// total box width with the corners.
pub const BOX_WIDTH: i32 = BOX_INNER_WIDTH as i32 + 2; // 2 corners.
pub const BOX_HEIGHT: i32 = 5;

// Character set:
//
// vertical bars │ │ │
// separator ──────
//
// ┌──────┬──────┐
//
// ├──────┼──────┤
//
// └──────┴──────┘

impl Object {
    // Returns a centered number with whitespace as padding with the given width.
    pub fn fmt_width(&self, width: usize) -> String {
        // If this is a block then fix its width by offsetting it off of its
        // painted string.
        // colored strings are longer than they look in the terminal which is
        // why we must offset the width.
        if let Object::Block(number) = self {
            // Paint the number to its color.
            let painted = format!("{}", self.color().paint(number.to_string()));
            let offset = painted.len() - number.to_string().len();
            return format!("{: ^width$}", painted, width = width + offset);
        }
        format!("{: ^width$}", "", width = width)
    }
    // write_box draws this object as a box in a String.
    pub fn write_box(&self, x: i32, y: i32) -> String {
        let mut response = String::new();
        let color = self.color();
        // Move the cursor to the specified x and y coordinates.
        // response.push_str(&format!("\x1B[{};{}H", x, y));
        // Loop through the box height and begin drawing the box's rows.
        for xx in 0..BOX_HEIGHT + 1 {
            let msg: String = match xx {
                // If this is the first or last match.
                1 | BOX_HEIGHT => {
                    // Get the left and right corners based on what the xx is.
                    let (lc, rc) = if xx == 1 {
                        ("┌", "┐")
                    } else {
                        ("└", "┘")
                    };
                    // Create a center separator for the corners.
                    let cnt = "─".repeat(BOX_INNER_WIDTH);
                    // Paint and return as string.
                    format!("{}{}{}", color.paint(lc), color.paint(cnt), color.paint(rc))
                }
                // Default case.
                _ => {
                    // If we are in the middle of the box, then draw the box's
                    // number or draw empty if not.
                    let obj = if xx == (BOX_HEIGHT + 1) / 2 {
                        *self // Copy value of our object.
                    } else {
                        Object::Empty
                    };
                    format!(
                        "{0}{1}{0}",
                        color.paint("│"),
                        obj.fmt_width(BOX_INNER_WIDTH)
                    )
                }
            };
            // push the part of this box into the string response.
            response.push_str(&msg);
            // Move the cursor to the next position.
            response.push_str(&format!("\x1B[{};{}H", xx + x, y));
        }
        response
    }
    // Returns a color value dependent on the object's number value.
    pub fn color(&self) -> Color {
        if let Object::Block(num) = self {
            // Match the number value of the block.
            match num {
                2 => return Color::RGB(255, 80, 0),
                4 => return Color::RGB(255, 165, 0),
                8 => return Color::RGB(255, 200, 0),
                16 => return Color::RGB(255, 255, 0),
                32 => return Color::RGB(128, 200, 0),
                64 => return Color::RGB(0, 128, 0),
                128 => return Color::RGB(0, 85, 128),
                256 => return Color::RGB(0, 0, 255),
                512 => return Color::RGB(64, 0, 225),
                1024 => return Color::RGB(75, 0, 130),
                2048 => return Color::RGB(230, 110, 210),
                _ => return Color::RGB(255, 150, 200),
            }
        }
        // If it's empty then return black.
        Color::White
    }
}
