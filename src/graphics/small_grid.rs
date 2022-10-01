use frames::frame_types::basic;
use frames::modifiers::position;
use frames::prelude::*;
use frames::layout_manager::*;
use basic::IBasic;

use crate::prelude::*;
use crate::state::Node;

use super::num_char;

pub struct SmallGrid {
    frame:      basic::Basic,
    pos:        position::Position,
    sub_size:   Coord,
    size:       Coord,
    pointer:    Coord,
    pointer_on: bool,
}

impl SmallGrid {
    pub fn new(man: &mut LayoutManager, sub_size: Coord) -> SmallGrid {
        let size = Coord{x: 0, y: 0};
        let frame = basic::new(size, Vec::new()).unwrap();
        let pos = position::craft().pos(Coord { x: 0, y: 1 }).size(size).done();

        man.layout.borrow_mut()
            .objects.push(Object { frame: frame.clone(), pos: pos.clone() });
        
        let mut temp = SmallGrid {
            frame,
            pos,
            sub_size,
            size,
            pointer: Coord { x: 0, y: 0 },
            pointer_on: false,
        };

        temp.resize(sub_size);
        temp
    }

    pub fn resize(&mut self, sub_size: Coord) {
        let size = Coord {
            x: 1 + ((sub_size.x + 1) * sub_size.y),
            y: 1 + ((sub_size.y + 1) * sub_size.x),
        };
        let area = size.x * size.y;
        let colors = ColorSet {
            fg: Color::White,
            bg: Color::Black,
        };

        let mut borrowed = self.frame.borrow_mut();
        borrowed.replace(size, vec![Pixel::Clear; area as usize]).unwrap();

        //corners
        {
            let x = size.x - 1;
            let y = size.y - 1;

            borrowed.set_pixel(Coord{x: 0, y: 0}, Pixel::new_color_set('┌', colors));
            borrowed.set_pixel(Coord{x: 0, y: y}, Pixel::new_color_set('└', colors));
            borrowed.set_pixel(Coord{x: x, y: 0}, Pixel::new_color_set('┐', colors));
            borrowed.set_pixel(Coord{x: x, y: y}, Pixel::new_color_set('┘', colors));
        }
        
        //rows
        draw_row(&mut borrowed, colors, size.x, sub_size, 0, '─', '┬');
        draw_row(&mut borrowed, colors, size.x, sub_size, size.y - 1, '─', '┴');

        for y in 1..(size.y - 1) {
            if y % (sub_size.y + 1) != 0 {
                draw_row(&mut borrowed, colors, size.x, sub_size, y, ' ', '│');
            }
            else {
                draw_row(&mut borrowed, colors, size.x, sub_size, y, '─', '┼');
            };
            
        }

        //columns
        draw_col(&mut borrowed, colors, size.y, sub_size, 0, '│', '├');
        draw_col(&mut borrowed, colors, size.y, sub_size, size.x - 1, '│', '┤');

        self.pos.borrow_mut().data.size = size;
        self.size = size;
        self.sub_size = sub_size;
    }

    pub fn get_size(&self) -> Coord {
        self.sub_size
    }

    pub fn enabled(&mut self, enabled: bool) {
        self.pos.borrow_mut().data.enabled = enabled;
    }

    fn translate(&self, pos: Coord) -> Coord {
        Coord {
            x: 1 + pos.x + (pos.x / (self.sub_size.x)),
            y: 1 + pos.y + (pos.y / (self.sub_size.y))
        }
    }

    pub fn update(&mut self, nodes: &Vec2D<Node>) {
        let mut frame = self.frame.borrow_mut();

        for pos in CoordIter::new(Coord { x: 0, y: 0 }, nodes.size()) {
            let loc_pos = self.translate(pos);
            let node = nodes.get(pos);
            if node.is_found() {
                frame.set_char(loc_pos, num_char(node.get_num().unwrap()));

                if node.has_conflicts() {
                    frame.set_colors(loc_pos, ColorSet { fg: Color::Red, bg: Color::Black });
                }
                else {
                    frame.set_colors(loc_pos, ColorSet { fg: Color::White, bg: Color::Black });
                }
            }
            else {
                frame.set_char(loc_pos, ' ');
                frame.set_colors(loc_pos, ColorSet { fg: Color::White, bg: Color::Black });
            }
        }

        if self.pointer_on {
            flip_colors(&mut frame, self.translate(self.pointer));
        }
    }

    pub fn move_by(&mut self, amount: Coord) {
        self.pos.borrow_mut().data.pos += amount;
    }

    pub fn move_for_pointer(&mut self, mut size: Coord) {
        let mut pos = self.pos.borrow_mut();

        size.x -= super::LIST_SIZE + 2;
        size.y -= 2;
        let pointer = self.translate(self.pointer) + pos.data.pos;

        let shift = Coord {
            x: to_range(pointer.x, 1, size.x),
            y: to_range(pointer.y, 2, size.y),
        };

        pos.data.pos += shift;
    }

    pub fn pointer(&self) -> Coord {
        self.pointer
    }

    pub fn pointer_on(&mut self, size: Coord) {
        self.move_for_pointer(size);
        let mut frame = self.frame.borrow_mut();
        let pos = self.translate(self.pointer);
        self.pointer_on = true;

        flip_colors(&mut frame, pos);
    }

    pub fn pointer_off(&mut self) {
        let mut frame = self.frame.borrow_mut();
        let pos = self.translate(self.pointer);
        self.pointer_on = false;

        flip_colors(&mut frame, pos);
    }

    pub fn set_pointer(&mut self, pos: Coord, size: Coord) {
        if self.pointer_on {
            let mut frame = self.frame.borrow_mut();
            let old_pos = self.translate(self.pointer);
            let new_pos = self.translate(pos);

            flip_colors(&mut frame, old_pos);
            flip_colors(&mut frame, new_pos);
        }

        self.pointer = pos;
        self.move_for_pointer(size);
    }

    pub fn inc_pointer(&mut self, amount: Coord, screen_size: Coord) {
        let size = self.sub_size.x * self.sub_size.y;
        let pos = Coord {
            x: (((self.pointer.x + amount.x) % size) + size) %size,
            y: (((self.pointer.y + amount.y) % size) + size) %size,
        };
        
        self.set_pointer(pos, screen_size);
    }
}

fn draw_row(basic: &mut IBasic, colors: ColorSet, size: i32, sub_size: Coord, y: i32, main: char, alt: char) {
    for x in 1..(size - 1) {
        let t = if x % (sub_size.x + 1) != 0 { main }
                                              else { alt };

        basic.set_pixel(Coord{x, y}, Pixel::new_color_set(t, colors));
    }
}

fn draw_col(basic: &mut IBasic, colors: ColorSet, size: i32, sub_size: Coord, x: i32, main: char, alt: char) {
    for y in 1..(size - 1) {
        let t = if y % (sub_size.y + 1) != 0 { main }
                                              else { alt };

        basic.set_pixel(Coord{x, y}, Pixel::new_color_set(t, colors));
    }
}

fn flip_colors(frame: &mut IBasic, pos: Coord) {
    if let Pixel::Opaque(data) = frame.get_pixel(pos) {
        frame.set_colors(pos, ColorSet { fg: data.bg, bg: data.fg })
    }
}

fn to_range(pos: i32, min: i32, max: i32) -> i32 {
    if pos < min {
        return min - pos
    }
    
    if pos > max {
        return max - pos
    }

    0
}