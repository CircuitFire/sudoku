use frames::frame_types::text::Entry;
use frames::prelude::*;
use frames::layout_manager::*;
use frames::ManagerTrait;
use frames::frame_types::text;
use frames::crossterm::event::KeyCode;

pub struct Help {
    frame: text::Text,
    pos:   position::Position,
}

impl Help {
    pub fn new(man: &mut LayoutManager) -> Self {
        let frame = text::new();
        {
            let mut borrowed = frame.borrow_mut();
    
            borrowed.indent = text::Indent::Hanging(4);
            borrowed.default.bg = super::BORDER;
            borrowed.default.fg = Color::White;

            borrowed.entries.push_back(Entry::new(
                "Help Menu:\
                \nYou are currently here. Provides descriptions for each function.
                \nEsc: Exit\
                \nArrow Keys: scroll menu\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "General Help:\
                \nIf the text is too small change the terminals font size by right clicking the top bar and clicking properties.\
                \nWhen resizing the window pressing any key will redraw the screen.\
                \nIf a solver results in conflicts then it means that there was already a problem present in the puzzle.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Main Menu:\
                \nGeneral overview of the puzzle and where larger choices are provided.
                \nEsc: Exit\
                \nArrow Keys: move puzzle\
                \n+: Increase guess Level\
                \n-: Decrease guess Level\
                \nI: Enter Insertion Mode\
                \nB: Basic Solve\
                \nShift + B: Loop Basic Solve\
                \nE: Exclusive Solve\
                \nShift + E: Loop Exclusive Solve\
                \nF: Full Solve Without Guessing\
                \nShift F: Full Solve With Guessing\
                \nC: Check if puzzle is currently possible.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Insertion Mode:\
                \nUsed to manually Solve nodes of the puzzle.
                \nArrow Keys: Move pointer location\
                \nShift + Arrow Keys: Move puzzle\
                \n+: Increase guess Level\
                \n-: Decrease guess Level\
                \n1-9, A-P: Solve node under the pointer with the selected number\\letter\
                \nBackspace: Clear a solved node\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Guess Level:\
                \nGuess level makes making guesses more convenient.\
                \nBefore making a guess increase the guess level and then continue solving the puzzle.\
                \nIf you are not happy with the guess you can decrease the guess level which automatically clear all nodes that where dependent on that guess.\
                \nYou can have as many guess levels as you want.\
                \nThe auto solver 'Auto Guess' uses this feature.\
                \nIt's recommended to use guess level 0 for the given numbers of the puzzle so you can easy reset.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Basic Solve:\
                \nThe basic solve simply looks at the possibilities of each node and if it only has one possibility it will solve it.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Exclusive Solve:\
                \nExclusive solve looks though the nodes of each row column and block and check if only one node can be a solution it solves it as that.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "Full Solve:\
                \nFull solve uses all of the available solves to complete the puzzle.\
                \nNormal mode will not make it's own guesses.\
                \nPressing shift 'F' will allow it to also make guesses, and if the puzzle if possible should completely solve it.\
                \n"
            ));

            borrowed.entries.push_back(Entry::new(
                "About:\
                \nSource Code: https://github.com/CircuitFire/sudoku
                \n"
            ));
        }
    
        let pos = position::craft()
            .size(man.size())
            .update(position::update_types::MatchSize{})
            .enabled(false)
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

    pub fn main(&mut self, man: &mut LayoutManager) {
        self.pos.borrow_mut().data.enabled = true;
        
        loop {
            man.draw().unwrap();

            if let Input::KeyBoard(x) = man.get_input() {
                match x.code {
                    KeyCode::Esc   => { break; },
                    KeyCode::Up    => { self.pos.borrow_mut().data.offset.x -= 1 },
                    KeyCode::Down  => { self.pos.borrow_mut().data.offset.x += 1 },
                    _ => {}
                }
            }
        }

        self.pos.borrow_mut().data.enabled = false;
    }
}








