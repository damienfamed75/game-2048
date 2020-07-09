// standard libraries
use std::fmt;
use std::fmt::Display;
// internal modules
use crate::grid::{Grid, GRID_WIDTH};

const BOX_WIDTH: usize = 8; // individual box width

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

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.to_string()) }
}

impl Grid {
    fn to_string(&self) -> String {
        let arr = self.0;
        let mut response = String::new();
        // Append the top of the grid.
        response.push_str(&format!(
            "┌{0:─>width$}┬{0:─>width$}┬{0:─>width$}┬{0:─>width$}┐",
            "", // intentional empty string
            width = BOX_WIDTH
        ));

        for x in 0..GRID_WIDTH {
            response.push_str("\n│");
            response.push_str(&format!("{: >width$}│", "", width = BOX_WIDTH).repeat(arr[x].len()));
            response.push_str("\n│");

            // Write the formatted object to the string.
            for obj in arr[x].iter() {
                // each iteration adds: "{num}   │"
                response.push_str(&obj.fmt_width(BOX_WIDTH));
                response.push('│');
            }

            response.push_str("\n│");
            response.push_str(&format!("{: >width$}│", "", width = BOX_WIDTH).repeat(arr[x].len()));
            // If this isn't the last line, then draw connectors between all of
            // the boxes.
            if x != GRID_WIDTH - 1 {
                response.push_str(&format!("\n├{:─>width$}┼", "─", width = BOX_WIDTH));
                response.push_str(
                    &format!("{:─>width$}┼", "─", width = BOX_WIDTH).repeat(arr[x].len() - 2),
                );
                response.push_str(&format!("{:─>width$}┤", "─", width = BOX_WIDTH));
            }
        }
        // Append the bottom of the grid.
        response.push_str(&format!(
            "\n└{0:─>width$}┴{0:─>width$}┴{0:─>width$}┴{0:─>width$}┘",
            "", // intentional empty string
            width = BOX_WIDTH
        ));
        response
    }
}
