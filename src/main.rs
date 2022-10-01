use frames::{prelude::*, ManagerTrait};
use frames::layout_manager::{LayoutManager, Object, position};
use frames::frame_types::fill;
use frames::crossterm;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::KeyCode;

use sudoku::graphics::{SmallGrid, List};
use sudoku::game::Game;

fn main() {
    enable_raw_mode().unwrap();

    let mut manager = LayoutManager::new().unwrap();
    manager.set_debug(true);

    manager.layout.borrow_mut().objects.push(Object{
        frame: fill::new(Pixel::new(' ', Color::White, Color::Black)),
        pos: position::craft().update(position::update_types::MatchSize{}).done()
    });

    let mut grid = SmallGrid::new(&mut manager, Coord { x: 3, y: 3 });
    let mut list = List::new(&mut manager);
    list.show_size(Coord { x: 3, y: 3 });

    set_size(&mut manager, &mut grid, &mut list);
    
    let mut game = Game::new(&mut manager, grid, list);
    
    game.main(&mut manager);

    disable_raw_mode().unwrap();
}

pub fn set_size(manager: &mut LayoutManager, grid: &mut SmallGrid, list: &mut List) {
    loop {
        manager.draw().unwrap();

        if let Input::KeyBoard(x) = manager.get_input() {
            match x.code {
                KeyCode::Esc   => { break; },
                KeyCode::Enter => { break; },
                KeyCode::Left  => { change_size(grid, list, Coord { x:  0, y: -1 }) },
                KeyCode::Right => { change_size(grid, list, Coord { x:  0, y:  1 }) },
                KeyCode::Up    => { change_size(grid, list, Coord { x: -1, y:  0 }) },
                KeyCode::Down  => { change_size(grid, list, Coord { x:  1, y:  0 }) },
                _ => {}
            }
        }
    }
}

fn change_size(grid: &mut SmallGrid, list: &mut List, change: Coord) {
    let new = grid.get_size() + change;

    let check = new.x * new.y;
    if 0 < check && check <= 25  {
        grid.resize(new);
        list.show_size(new);
    }
}
