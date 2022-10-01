use frames::frame_types::text::Entry;
use frames::prelude::*;
use frames::layout_manager::*;
use frames::frame_types::text;
use frames::frame_types::basic::{self, IBasic};

const TEXT_SIZE: Coord = Coord { x: 40, y: 5 };
const BOX_SIZE: Coord = Coord { x: 42, y: 7 };

struct CenterUpdate {}

impl position::SizeUpdate for CenterUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, new_size: Coord) {
        pos.pos = center(new_size, pos.size);
    }
}

pub struct PopUp {
    text_box:   text::Text,
    text_pos:   position::Position,
    border:     basic::Basic,
    border_pos: position::Position,
}

impl PopUp {
    pub fn new(man: &mut LayoutManager) -> Self {
        let text = text::new();
        {
            let mut borrowed = text.borrow_mut();

            borrowed.default.bg = super::BORDER;
            borrowed.default.fg = Color::White;

            borrowed.entries.push_back(Entry::new(""));
            borrowed.entries.push_back(Entry::new(""));
            borrowed.entries.push_back(Entry::new(""));
        }
        
        let text_pos = position::craft()
            .size(TEXT_SIZE)
            .pos(center(man.size(), TEXT_SIZE))
            .update(CenterUpdate{})
            .enabled(false)
            .done();

        let box_frame = basic::new(BOX_SIZE, vec![Pixel::Clear; (BOX_SIZE.x * BOX_SIZE.y) as usize]).unwrap();

        let colors = ColorSet {
            fg: Color::White,
            bg: Color::Black,
        };

        {
            let mut borrowed = box_frame.borrow_mut();
            let x = BOX_SIZE.x - 1;
            let y = BOX_SIZE.y - 1;

            borrowed.set_pixel(Coord{x: 0, y: 0}, Pixel::new_color_set('┌', colors));
            borrowed.set_pixel(Coord{x: 0, y: y}, Pixel::new_color_set('└', colors));
            borrowed.set_pixel(Coord{x: x, y: 0}, Pixel::new_color_set('┐', colors));
            borrowed.set_pixel(Coord{x: x, y: y}, Pixel::new_color_set('┘', colors));

            draw_row(&mut borrowed, colors, x,0, '─');
            draw_row(&mut borrowed, colors, x,  y, '─');

            draw_col(&mut borrowed, colors, y, 0, '│');
            draw_col(&mut borrowed, colors, y,   x, '│');
        }

        let box_pos = position::craft()
            .size(BOX_SIZE)
            .pos(center(man.size(), BOX_SIZE))
            .update(CenterUpdate{})
            .enabled(false)
            .done();
        
        {
            let mut borrowed = man.layout.borrow_mut();

            borrowed.objects.push(Object{
                frame: text.clone(),
                pos: text_pos.clone()
            });
            borrowed.objects.push(Object{
                frame: box_frame.clone(),
                pos: box_pos.clone()
            });
        }
        
        Self {
            text_box:   text,
            text_pos:   text_pos,
            border:     box_frame,
            border_pos: box_pos,
        }
    }

    pub fn possible(&mut self, possible: bool) {
        self.border_pos.borrow_mut().data.enabled = true;
        self.text_pos.borrow_mut().data.enabled   = true;

        self.text_box.borrow_mut().entries[0].set_text(
            format!("\n{:^width$}\n",
                if possible {"Currently Possible."}
                else {"Not Currently Possible."},
                width = (TEXT_SIZE.x - 1) as usize
            )
        );
        self.text_box.borrow_mut().entries[1].set_text(
            format!("{:^width$}", "[Any Key to continue]", width = TEXT_SIZE.x as usize)
        );
        self.text_box.borrow_mut().entries[2].set_text("");
    }

    pub fn leaving(&mut self) {
        self.border_pos.borrow_mut().data.enabled = true;
        self.text_pos.borrow_mut().data.enabled   = true;

        self.text_box.borrow_mut().entries[0].set_text(
            format!("\n{:^width$}\n",
                "Exit Program?",
                width = (TEXT_SIZE.x - 1) as usize
            )
        );
        self.text_box.borrow_mut().entries[1].set_text(
            format!("{:^width$}", "[Esc to Exit]", width = TEXT_SIZE.x as usize)
        );
        self.text_box.borrow_mut().entries[2].set_text(
            format!("{:^width$}", "[Enter to Continue]", width = TEXT_SIZE.x as usize)
        );
    }

    pub fn disable(&mut self) {
        self.border_pos.borrow_mut().data.enabled = false;
        self.text_pos.borrow_mut().data.enabled   = false;
    }
}

fn draw_row(basic: &mut IBasic, colors: ColorSet, size: i32, y: i32, c: char) {
    for x in 1..size {
        basic.set_pixel(Coord{x, y}, Pixel::new_color_set(c, colors));
    }
}

fn draw_col(basic: &mut IBasic, colors: ColorSet, size: i32, x: i32, c: char) {
    for y in 1..size {
        basic.set_pixel(Coord{x, y}, Pixel::new_color_set(c, colors));
    }
}

fn center(screen: Coord, size: Coord)-> Coord {
    (screen / Coord::same(2)) - (size / Coord::same(2))
}