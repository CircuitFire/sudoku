
mod small_grid;
pub use small_grid::SmallGrid;

mod list;
pub use list::{List, LIST_SIZE};

mod status_bar;
pub use status_bar::StatusBar;

mod help;
pub use help::Help;

mod popup;
pub use popup::PopUp;

use frames::prelude::Color;
pub const BORDER: Color = Color::Rgb { r: 20, g: 20, b: 20 };

pub fn num_char(num: usize) -> char {
    match num {
        0..=32 => {
            char::from_digit((num + 1) as u32, 32).unwrap().to_ascii_uppercase()
        }
        _ => {
            '?'
        }
    }
}