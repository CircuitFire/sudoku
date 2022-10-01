use frames::frame_types::text::Entry;
use frames::prelude::*;
use frames::layout_manager::*;
use frames::frame_types::text;

use crate::state::Node;

use super::num_char;

pub const LIST_SIZE: i32 = 26;

struct ListUpdate {}

impl position::SizeUpdate for ListUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, new_size: Coord) {
        pos.pos.x = new_size.x - LIST_SIZE;
        pos.size.y = new_size.y;
    }
}

pub struct List {
    frame: text::Text,
    pos:   position::Position,
}

impl List {
    pub fn new(man: &mut LayoutManager) -> Self {
        let frame = text::new();
        {
            let mut borrowed = frame.borrow_mut();

            borrowed.default.bg = super::BORDER;
            borrowed.default.fg = Color::White;
        }

        let pos = position::craft()
            .size(Coord { x: LIST_SIZE, y: 0 })
            .update(ListUpdate{})
            .done();

        man.layout.borrow_mut().objects.push(Object{
            frame: frame.clone(),
            pos: pos.clone()
        });
        
        Self {
            frame,
            pos,
        }
    }

    pub fn node(&mut self, pos: Coord, node: &Node) {
        let mut frame = self.frame.borrow_mut();
        frame.entries.clear();

        frame.indent = text::Indent::Hanging(2);

        frame.entries.push_back(Entry::new(format!(
            "Node: (y: {}, x: {})\n", pos.y, pos.x
        )));

        match &node {
            Node::Found(data) => {
                frame.entries.push_back(Entry::new(format!(
                    "Solved: {}", num_char(data.num)
                )));
                frame.entries.push_back(Entry::new(format!(
                    "Guess Level: {}\n", data.guess_level
                )));

                let mut temp = "Conflicts with:".to_string();

                for conf in &data.conflicts {
                    temp.push_str(&format!("\n(y: {}, x: {})", conf.y , conf.x))
                }

                frame.entries.push_back(Entry::new( temp ));
            },
            Node::Possible(list) => {
                frame.entries.push_back(Entry::new(
                    "Unsolved:\n"
                ));

                let mut temp = "Possible Solutions:\n".to_string();

                let mut iter = list.iter()
                    .enumerate()
                    .filter(|x| *x.1);

                if let Some((num, _)) = iter.next() {
                    temp.push(num_char(num));
                }

                for (num, _) in iter {
                    temp.push_str(&format!(", {}", num_char(num)))
                }

                frame.entries.push_back(Entry::new( temp ));
            },
        }
    }

    pub fn show_size(&mut self, sub_size: Coord) {
        let mut frame = self.frame.borrow_mut();
        frame.entries.clear();

        let size = sub_size.x * sub_size.y;

        frame.entries.push_back(Entry::new(
            "Set puzzle size with the\narrow keys and press\nenter key to continue.
            \nIf you are new you should\nread the help menu.\n"
        ));
        frame.entries.push_back(Entry::new(
            format!("Size: {}x{}", size, size)
        ));
        frame.entries.push_back(Entry::new(
            format!("Block size: {}x{}", sub_size.x, sub_size.y)
        ));
    }

    pub fn main(&mut self) {
        let mut frame = self.frame.borrow_mut();
        frame.entries.clear();
        frame.indent = text::Indent::Hanging(2);

        frame.entries.push_back(Entry::new(
            "Esc:\nExit"
        ));
        frame.entries.push_back(Entry::new(
            "H:\nHelp Menu"
        ));
        frame.entries.push_back(Entry::new(
            "Arrow keys:\nMove Puzzle"
        ));
        frame.entries.push_back(Entry::new(
            "I:\nInsertion Mode"
        ));
        frame.entries.push_back(Entry::new(
            "B (Shift Loop):\nBasic Solve"
        ));
        frame.entries.push_back(Entry::new(
            "E (Shift Loop):\nExclusive Solve"
        ));
        frame.entries.push_back(Entry::new(
            "F:\nFull Solve no guessing"
        ));
        frame.entries.push_back(Entry::new(
            "F + Shift:\nFull Solve"
        ));
        frame.entries.push_back(Entry::new(
            "C:\nCheck if possible"
        ));
        
    }

    pub fn enabled(&mut self, enabled: bool) {
        self.pos.borrow_mut().data.enabled = enabled;
    }
}