use crate::prelude::*;

pub struct Finished {
    current: usize,
    total:   usize,
    rows:    Vec<usize>,
    columns: Vec<usize>,
    blocks:  Vec2D<usize>,
}

impl Finished {
    pub fn new(size: usize, blocks: Coord) -> Self {
        Self {
            current: 0,
            total:   size * size,
            rows:    vec![0; size],
            columns: vec![0; size],
            blocks:  Vec2D::new(blocks, 0),
        }
    }

    pub fn inc(&mut self, pos: Coord) {
        self.current += 1;
        self.rows[pos.y as usize] += 1;
        self.columns[pos.x as usize] += 1;
        *self.blocks.get_mut(pos / self.blocks.size()) += 1;
    }

    pub fn dec(&mut self, pos: Coord) {
        self.current -= 1;
        self.rows[pos.y as usize] -= 1;
        self.columns[pos.x as usize] -= 1;
        *self.blocks.get_mut(pos / self.blocks.size()) -= 1;
    }

    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn get_total(&self) -> usize {
        self.total
    }

    pub fn done(&self) -> bool {
        self.current == self.total
    }
}