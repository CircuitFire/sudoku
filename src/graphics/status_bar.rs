use frames::frame_types::text::Entry;
use frames::prelude::*;
use frames::layout_manager::*;
use frames::ManagerTrait;
use frames::frame_types::text;

use crate::state::State;

struct StatusBarUpdate {}

impl position::SizeUpdate for StatusBarUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, new_size: Coord) {
        pos.size.x = new_size.x - super::LIST_SIZE;
    }
}

pub struct StatusBar {
    frame: text::Text,
    pos:   position::Position,
}

impl StatusBar {
    pub fn new(man: &mut LayoutManager) -> Self {
        let frame = text::new();
        {
            let mut borrowed = frame.borrow_mut();
    
            borrowed.default.bg = super::BORDER;
            borrowed.default.fg = Color::White;

            borrowed.entries.push_back(Entry::new(""));
        }
    
        let pos = position::craft()
            .size(Coord{ x: man.size().x - super::LIST_SIZE, y: 1 })
            .update(StatusBarUpdate{})
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
    
    pub fn enabled(&mut self, enabled: bool) {
        self.pos.borrow_mut().data.enabled = enabled;
    }

    pub fn update(&mut self, state: &State) {
        self.frame.borrow_mut().entries[0].set_text(
            format!{"Guess Level {} | Completion {} / {}",
            state.guess_level(),
            state.finished(),
            state.total(),
        })
    }
}
