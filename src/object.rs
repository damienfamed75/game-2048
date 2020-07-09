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
        Color::Black
    }
}
