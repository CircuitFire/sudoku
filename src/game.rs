use crate::prelude::*;
use crate::graphics::{SmallGrid, List, StatusBar, Help, PopUp};
use crate::state::State;

use frames::{prelude::*, ManagerTrait};
use frames::layout_manager::LayoutManager;
use frames::crossterm::event::{KeyCode, KeyModifiers};

pub struct Game {
    grid:   SmallGrid,
    status: StatusBar,
    help:   Help,
    list:   List,
    state:  State,
    popup:  PopUp,
}

impl Game {
    pub fn new(manager: &mut LayoutManager, grid: SmallGrid, list: List) -> Self {
        Self {
            state:  State::new(grid.get_size()),
            status: StatusBar::new(manager),
            help:   Help::new(manager),
            popup:  PopUp::new(manager),
            grid,
            list,
        }
    }

    pub fn main(&mut self, manager: &mut LayoutManager) {
        self.list.main();
        self.status.update(&self.state);

        loop {
            manager.draw().unwrap();

            if let Input::KeyBoard(x) = manager.get_input() {
                use KeyCode::*;
                match x {
                    KeyEvent{code: Esc, ..}   => {
                        self.popup.leaving();
                        manager.draw().unwrap();
                        if self.exit(manager) {
                            break;
                        }
                        self.popup.disable();
                    },
                    KeyEvent{code: Left, ..}  => { self.grid.move_by(Coord { x: -1, y:  0 }) },
                    KeyEvent{code: Right, ..} => { self.grid.move_by(Coord { x:  1, y:  0 }) },
                    KeyEvent{code: Up, ..}    => { self.grid.move_by(Coord { x:  0, y: -1 }) },
                    KeyEvent{code: Down, ..}  => { self.grid.move_by(Coord { x:  0, y:  1 }) },
                    KeyEvent{code: Char(c), ..} => {
                        match c {
                            '-' | '_' => {
                                self.dec_guess();
                            }
                            '=' | '+' => {
                                self.inc_guess();
                            }
                            'h' | 'H' => {
                                self.help(manager)
                            }
                            'i' | 'I' => {
                                self.insert_mode(manager);
                                self.list.main();
                            }
                            'B' => {
                                self.state.loop_basic();
                                self.main_update();
                            }
                            'b' => {
                                self.state.solve_basic();
                                self.main_update();
                            }
                            'E' => {
                                self.state.loop_exclusive();
                                self.main_update();
                            }
                            'e' => {
                                self.state.solve_exclusive();
                                self.main_update();
                            }
                            'f' => {
                                self.state.full_solve_no_guessing();
                                self.main_update();
                            }
                            'F' => {
                                self.state.full_solve();
                                self.main_update();
                            }
                            'c' | 'C' => {
                                self.popup.possible(self.state.check_if_possible());
                                manager.draw().unwrap();
                                manager.get_input();
                                self.popup.disable();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn exit(&mut self, manager: &mut LayoutManager) -> bool {
        loop {
            manager.draw().unwrap();

            if let Input::KeyBoard(x) = manager.get_input() {
                use KeyCode::*;
                match x.code {
                    Esc =>   { return true; },
                    Enter => { return false; }
                    _ => {}
                }
            }
        }
    }

    fn help(&mut self, manager: &mut LayoutManager) {
        self.grid.enabled(false);
        self.status.enabled(false);
        self.list.enabled(false);

        self.help.main(manager);

        self.grid.enabled(true);
        self.status.enabled(true);
        self.list.enabled(true);
    }

    pub fn insert_mode(&mut self, manager: &mut LayoutManager) {
        self.grid.pointer_on(manager.size());
        self.update_list();

        loop {
            manager.draw().unwrap();

            if let Input::KeyBoard(x) = manager.get_input() {
                use KeyCode::*;
                match x {
                    KeyEvent{code: Esc, ..} => { break; },
                    KeyEvent{code: Left, modifiers: KeyModifiers::SHIFT, ..} => {
                        self.grid.move_by(Coord { x: -1, y:  0 })
                    }
                    KeyEvent{code: Left, ..} => {
                        self.set_pointer(Coord { x: -1, y:  0 }, manager.size())
                    },
                    KeyEvent{code: Right, modifiers: KeyModifiers::SHIFT, ..} => {
                        self.grid.move_by(Coord { x:  1, y:  0 })
                    }
                    KeyEvent{code: Right, ..} => {
                        self.set_pointer(Coord { x:  1, y:  0 }, manager.size())
                    },
                    KeyEvent{code: Up, modifiers: KeyModifiers::SHIFT, ..} => {
                        self.grid.move_by(Coord { x:  0, y: -1 })
                    }
                    KeyEvent{code: Up, ..} => {
                        self.set_pointer(Coord { x:  0, y: -1 }, manager.size())
                    },
                    KeyEvent{code: Down, modifiers: KeyModifiers::SHIFT, ..} => {
                        self.grid.move_by(Coord { x:  0, y:  1 })
                    }
                    KeyEvent{code: Down, ..} => {
                        self.set_pointer(Coord { x:  0, y:  1 }, manager.size())
                    },
                    KeyEvent{code: Backspace, ..} => {
                        self.clear();
                    }
                    KeyEvent{code: Char(c), ..} => {
                        match c {
                            '1'..='9' | 'a'..='w' => {
                                if let Some(num) = char_num(c) {
                                    if num < self.state.size() {
                                        self.set(num);
                                    }
                                }
                            }
                            '-' => {
                                self.dec_guess();
                            }
                            '=' => {
                                self.inc_guess();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        self.grid.pointer_off();
    }

    fn inc_guess(&mut self) {
        self.state.inc_guess();
        self.main_update()
    }

    fn dec_guess(&mut self) {
        self.state.dec_guess();
        self.main_update()
    }

    fn update_list(&mut self) {
        let pos = self.grid.pointer();

        self.list.node(
            pos,
            self.state.nodes().get(pos)
        )
    }

    fn set_pointer(&mut self, change: Coord, size: Coord) {
        self.grid.inc_pointer(change, size);
        self.update_list();
    }

    fn main_update(&mut self) {
        self.status.update(&self.state);
        self.grid.update(self.state.nodes());
    }

    fn insert_update(&mut self) {
        self.grid.update(self.state.nodes());
        self.update_list();
        self.status.update(&self.state);
    }

    fn set(&mut self, solve: usize) {
        self.state.set(self.grid.pointer(), solve);
        self.insert_update();
    }

    fn clear(&mut self) {
        self.state.clear(self.grid.pointer());
        self.insert_update();
    }
}

fn char_num(c: char) -> Option<usize> {
    match c {
        '1'..='9' | 'a'..='w' => { Some((c.to_digit(32).unwrap() - 1) as usize) }
        _ => { None }
    }
}