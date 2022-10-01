use crate::prelude::*;
use super::Finished;
use super::Node;


#[derive(Clone)]
enum Search {
    None,
    Found(Coord, usize),
    ToMany,
}

pub struct State {
    nodes:       Vec2D<Node>,
    finished:    Finished,
    guess_level: usize,
    guesses:     Vec<Coord>,
    size:        usize,
    blocks:      Coord,
}

impl State {
    pub fn new(sub_size: Coord) -> Self {
        let size = (sub_size.x * sub_size.y) as usize;
        let blocks = Coord {
            x: sub_size.y,
            y: sub_size.x
        };

        Self {
            nodes:       Vec2D::new(Coord {x: size as i32, y: size as i32}, Node::new(size as usize)),
            finished:    Finished::new(size, blocks),
            guess_level: 0,
            guesses:     Vec::new(),
            size,
            blocks,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn nodes(&self) -> &Vec2D<Node> {
        &self.nodes
    }

    pub fn guess_level(&self) -> usize {
        self.guess_level
    }

    pub fn finished(&self) -> usize {
        self.finished.get_current()
    }

    pub fn total(&self) -> usize {
        self.finished.get_total()
    }

    ///Set a node to be solved as the given number, with the current guess level.
    pub fn set(&mut self, pos: Coord, solve: usize) {
        if self.nodes.get(pos).is_found() {
            self.clear_conflicts(pos);
        }
        else {
            self.finished.inc(pos);
        }

        let node = self.nodes.get_mut(pos);

        node.set(solve, self.guess_level);

        self.set_conflicts(pos);
        self.reset_possible();
    }

    ///Removes the current solution from the node and recalculates the possibilities of the other nodes.
    pub fn clear(&mut self, pos: Coord) {
        self.clear_conflicts(pos);
        
        let node = self.nodes.get_mut(pos);

        *node = Node::new(self.size);

        if let Some(guess_id) = node.get_guess_id() {
            self.guesses.remove(guess_id);
        }

        self.finished.dec(pos);
        self.reset_possible();
    }

    pub fn inc_guess(&mut self) {
        self.guess_level += 1;
    }

    pub fn dec_guess(&mut self) {
        if self.guess_level == 0 { return }
        self.guess_level -= 1;

        for pos in self.all_iter() {
            if self.nodes.get_mut(pos).guess_reset(self.guess_level) {
                self.clear(pos);
            }
        }

        self.reset_possible();
    }

    fn pos_block(&self, pos: Coord) -> Coord {
        Coord { x: pos.x/self.blocks.x, y: pos.y/self.blocks.y }
    }

    fn all_iter(&self) -> CoordIter {
        CoordIter::new(
            Coord { x: 0, y: 0 },
            Coord { x: self.size as i32, y: self.size as i32 }
        )
    }

    fn row_iter(&self, y: i32) -> CoordIter {
        row_iter(self.size as i32, y)
    }

    fn column_iter(&self, x: i32) -> CoordIter {
        column_iter(self.size as i32, x)
    }

    fn block_iter(&self, pos: Coord) -> CoordIter {
        block_iter(self.blocks, pos)
    }

    fn iter_iter(&self) -> impl Iterator<Item = CoordIter> {
        iter_iter(self.size as i32, self.blocks)
    }

    fn point_iter(&self, pos: Coord) -> impl Iterator<Item = CoordIter> {
        [
            self.row_iter(pos.y),
            self.column_iter(pos.x),
            self.block_iter(self.pos_block(pos))
        ].into_iter()
    }

    fn reset_possible(&mut self) {
        for pos in self.all_iter() {
            self.nodes.get_mut(pos).reset_possible();
        }

        self.find_possible(self.iter_iter());
    }

    fn set_conflicts(&mut self, pos: Coord) {
        let num = self.nodes.get(pos).get_num();

        for iter in self.point_iter(pos) {
            for check in iter {
                if pos == check { continue; }

                if num == self.nodes.get(check).get_num() {
                    self.nodes.get_mut(pos).add_conflict(check);
                    self.nodes.get_mut(check).add_conflict(pos);
                }
            }
        }
    }

    fn clear_conflicts(&mut self, pos: Coord) {
        let (node, mut rest) = self.nodes.borrow_one(pos);

        if let Some(conflicts) = node.conflicts() {
            for conflict in conflicts {
                rest.get_mut(*conflict).remove_conflict(pos);
            }
        }
    }

    ///Check what solutions are possible for the nodes given in the range of the iterator.
    fn find_possible<T: Iterator<Item = CoordIter>>(&mut self, iter_iter: T) {
        let mut list = vec![true; self.size];

        for iter in iter_iter {
            list.fill(true);

            // get list of all solved in section.
            for pos in iter.clone() {
                let node = self.nodes.get(pos);
                
                if let Node::Found(data) = node {
                    list[data.num] = false;
                }
            }

            // set possible of all 
            for pos in iter {
                let node = self.nodes.get_mut (pos);

                if let Node::Possible(ref mut cur_list) = node {
                    for i in 0..list.len() {
                        if !list[i] {
                            cur_list[i] = false;
                        }
                    }
                }
            }
        }
    }

    ///Check possible solutions for nodes effected by one point.
    fn point_possibilities(&mut self, pos: Coord) {
        self.find_possible(self.point_iter(pos))
    }

    ///Try to solve unsolved nodes using there list of possibilities. returns true if any nodes were solved.
    pub fn solve_basic(&mut self) -> bool {
        let mut change = false;

        for pos in self.all_iter() {
            if let Some(_) = self.nodes.get_mut(pos).try_solve(self.guess_level) {
                self.finished.inc(pos);
                self.set_conflicts(pos);
                change = true;
            }
        }

        if change {
            self.find_possible(self.iter_iter());
        }
        
        change
    }

    pub fn loop_basic(&mut self) {
        while self.solve_basic() {}
    }

    ///Try to solve unsolved nodes by checking if they are the available solution for a group. returns true if any nodes were solved.
    pub fn solve_exclusive(&mut self) -> bool {
        let mut search = vec![Search::None; self.size];
        let mut change = false;

        for iter in self.iter_iter() {
            search.fill(Search::None);

            // find uniques
            for pos in iter {
                let node = self.nodes.get_mut(pos);

                if let Node::Possible(ref mut list) = node {
                    for i in 0..list.len() {
                        if list[i] {
                            match search[i] {
                                Search::None => {
                                    search[i] = Search::Found(pos, i);
                                }
                                Search::Found(_, _) => {
                                    search[i] = Search::ToMany;
                                }
                                Search::ToMany => {}
                            }
                        }
                    }
                }
            }

            // remove others from uniques
            for item in &search {
                if let Search::Found(pos, solve) = item {
                    self.set(*pos, *solve);
                    change = true;
                }
            }
        }
        change
    }

    pub fn loop_exclusive(&mut self) {
        while self.solve_exclusive() {}
    }

    fn has_conflicts(&self) -> bool {
        for pos in self.all_iter() {
            if self.nodes.get(pos).has_problems() { return true; }
        }

        false
    }

    ///return false if if encountered conflicts.
    pub fn full_solve_no_guessing(&mut self) -> bool {
        loop {
            if self.has_conflicts() { return false; }

            if self.solve_basic() { continue; }
            if self.solve_exclusive() { continue; }
            
            return true;
        }
    }

    fn guess_candidate(&self) -> Option<Coord> {
        if self.finished.done() { return None }

        let mut found_num = self.size + 1;
        let mut found_pos = None;

        for pos in self.all_iter() {
            if let Some(num) = self.nodes.get(pos).num_possibilities() {
                if num < found_num {
                    found_num = num;
                    found_pos = Some(pos);
                }
            }
        }

        found_pos
    }

    fn guess(&mut self) -> bool {
        if let Some(pos) = self.guess_candidate() {
            let mut solve = None;

            for (i, possible) in self.nodes.get(pos).possibilities().unwrap().iter().enumerate() {
                if *possible {
                    solve = Some(i);
                    break;
                }
            }

            let solve = solve.unwrap();

            self.inc_guess();

            self.set(pos, solve);
            self.nodes.get_mut(pos).set_guess_id(self.guesses.len());
            self.guesses.push(pos);

            true
        }
        else {
            false
        }
    }

    fn retry_guess(&mut self) -> bool {
        loop {
            if let Some(pos) = self.guesses.pop() {
                let last = self.nodes.get(pos).solution().unwrap();
                self.dec_guess();

                let mut solve = None;

                for (i, possible) in self.nodes.get(pos).possibilities().unwrap().iter().enumerate().skip(last + 1) {
                    if *possible {
                        solve = Some(i);
                        break;
                    }
                }

                if let Some(num) = solve {
                    self.inc_guess();
                    self.set(pos, num);
                    self.nodes.get_mut(pos).set_guess_id(self.guesses.len());
                    self.guesses.push(pos);
                }
                else {
                    continue;
                }

                return true
            }
            else {
                return false
            }
        }
    }

    pub fn full_solve(&mut self) -> bool {
        loop {
            if self.full_solve_no_guessing() {
                if self.guess() { continue; }
            }
            else {
                if self.retry_guess() { continue; }
            }

            return self.finished.done();
        }
    }

    pub fn check_if_possible(&mut self) -> bool {
        let guess = self.guess_level;

        self.inc_guess();
        let result = self.full_solve();

        while self.guess_level > guess {
            self.dec_guess();
        }

        result
    }
}

fn all_iter(size: i32) -> CoordIter {
    CoordIter::new(
        Coord { x: 0, y: 0 },
        Coord { x: size, y: size }
    )
}

fn row_iter(size: i32, y: i32) -> CoordIter {
    CoordIter::new(
        Coord { x: 0, y },
        Coord { x: size, y: y + 1 }
    )
}

fn column_iter(size: i32, x: i32) -> CoordIter {
    CoordIter::new(
        Coord { x, y: 0 },
        Coord { x: x + 1, y: size }
    )
}

fn block_iter(blocks: Coord, pos: Coord) -> CoordIter {
    let x = pos.x * blocks.y;
    let y = pos.y * blocks.x;

    CoordIter::new(
        Coord { x, y },
        Coord { x: x + blocks.y, y: y + blocks.x }
    )
}

fn iter_iter(size: i32, blocks: Coord) -> impl Iterator<Item = CoordIter> {
    let rows = (0..size).map(move |y| row_iter(size, y));
    let column = (0..size).map(move |x| column_iter(size, x));

    let blocks = CoordIter::new(
        Coord{ x: 0, y: 0,},
        blocks
    ).map(move |x| block_iter(blocks, x));

    rows.chain(column).chain(blocks)
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test1(){
        let mut state = State::new(Coord { x: 2, y: 2 });

        state.set(Coord { x: 0, y: 0 }, 1);
        //state.solve(Coord { x: 3, y: 3 }, 0);
        state.solve_basic();

        for pos in CoordIter::new(Coord { x: 0, y: 0 }, state.nodes().size()) {
            println!("{:?}: {:?}", pos, state.nodes().get(pos));
        }
    }

    #[test]
    fn test2(){
        let mut state = State::new(Coord { x: 2, y: 2 });

        let mut i = 0;
        for iter in state.iter_iter() {
            println!("iter #{}:", i);

            for pos in iter {
                println!("{:?}", pos);
            }

            i += 1;
        }
    }

    #[test]
    fn test3(){
        let mut state = State::new(Coord { x: 2, y: 2 });

        state.set(Coord { x: 0, y: 0 }, 1);
        state.set(Coord { x: 3, y: 0 }, 1);
        state.set(Coord { x: 0, y: 3 }, 1);
        
        println!("step one:");
        for pos in CoordIter::new(Coord { x: 0, y: 0 }, state.nodes().size()) {
            println!("{:?}: {:?}", pos, state.nodes().get(pos));
        }

        state.clear(Coord { x: 3, y: 0 });
        state.clear(Coord { x: 0, y: 3 });

        println!("step two:");
        for pos in CoordIter::new(Coord { x: 0, y: 0 }, state.nodes().size()) {
            println!("{:?}: {:?}", pos, state.nodes().get(pos));
        }
    }

    #[test]
    fn test4() {
        let mut state = State::new(Coord { x: 2, y: 2 });
        let pos = Coord { x: 0, y: 0 };

        state.inc_guess();
        state.set(pos, 1);

        println!("guess: {}, Node: {:?}", state.guess(), state.nodes().get(pos));

        state.dec_guess();

        println!("guess: {}, Node: {:?}", state.guess(), state.nodes().get(pos));

    }

    #[test]
    fn test5() {
        let mut state = State::new(Coord { x: 3, y: 3 });

        state.set(Coord { x: 0, y: 0 }, 6);

        state.set(Coord { x: 1, y: 1 }, 2);
        state.set(Coord { x: 2, y: 1 }, 3);
        state.set(Coord { x: 3, y: 1 }, 5);
        state.set(Coord { x: 8, y: 1 }, 0);

        state.set(Coord { x: 4, y: 2 }, 7);
        state.set(Coord { x: 7, y: 2 }, 1);

        state.set(Coord { x: 1, y: 3 }, 8);

        state.set(Coord { x: 1, y: 4 }, 0);
        state.set(Coord { x: 2, y: 4 }, 4);
        state.set(Coord { x: 3, y: 4 }, 2);
        state.set(Coord { x: 8, y: 4 }, 3);

        state.set(Coord { x: 5, y: 5 }, 5);
        state.set(Coord { x: 6, y: 5 }, 2);

        state.set(Coord { x: 0, y: 6 }, 5);
        state.set(Coord { x: 8, y: 6 }, 6);

        state.set(Coord { x: 1, y: 7 }, 4);
        state.set(Coord { x: 2, y: 7 }, 6);
        state.set(Coord { x: 5, y: 7 }, 1);
        state.set(Coord { x: 7, y: 7 }, 0);

        state.set(Coord { x: 0, y: 8 }, 8);
        state.set(Coord { x: 3, y: 8 }, 4);

        state.full_solve();
    }

    #[test]
    fn test6() {
        let mut state = State::new(Coord { x: 2, y: 2 });
        state.full_solve();
    }
}