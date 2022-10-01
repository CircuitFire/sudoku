pub use frames::prelude::Coord;
use std::iter::Iterator;

pub struct Vec2D<T> {
    buf: Vec<T>,
    size: Coord,
}

impl<T: Clone> Vec2D<T> {
    pub fn new(size: Coord, default: T) -> Self {
        Vec2D {
            buf: vec![default; (size.x * size.y) as usize],
            size
        }
    }

    fn flat(&self, pos: Coord) -> usize {
        ((pos.y * self.size.x) + pos.x) as usize
    }

    pub fn get_mut(&mut self, pos: Coord) -> &mut T {
        let i = self.flat(pos);
        &mut self.buf[i]
    }

    pub fn get(&self, pos: Coord) -> &T {
        &self.buf[self.flat(pos)]
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.buf
    }

    pub fn size(&self) -> Coord {
        self.size
    }

    ///return (borrowed, rest of list)
    pub fn borrow_one(&mut self, pos: Coord) -> (&mut T, SplitVec<T>) {
        let split = self.flat(pos);
        let len = self.buf.len();

        let (front, back) = self.buf.split_at_mut(split);

        if split < len - 1 {
            let (item, back) = back.split_at_mut(1);
            return (&mut item[0], SplitVec::new(self.size, split, front, Some(back)))
        }
        else {
            return (&mut back[0], SplitVec::new(self.size, split, front, None))
        }
    }
}

pub struct SplitVec<'a, T> {
    size:  Coord,
    split: usize,
    front: &'a mut [T],
    back:  Option<&'a mut [T]>,
}

impl<'a, T> SplitVec<'a, T> {
    fn new(size: Coord, split: usize, front: &'a mut [T], back: Option<&'a mut [T]>) -> Self {
        Self { size, split, front, back }
    }

    fn flat(&self, pos: Coord) -> usize {
        ((pos.y * self.size.x) + pos.x) as usize
    }

    pub fn get_mut(&mut self, pos: Coord) -> &mut T {
        let mut i = self.flat(pos);

        if i < self.front.len() {
           return &mut self.front[i]
        }

        if let Some(ref mut back) = self.back {
            i -= self.front.len() + 1;
            return &mut back[i]
        }

        panic!("Position out of bounds.")
    }
}

#[derive(Clone, Copy)]
pub struct CoordIter {
    cur:   Coord,
    start: Coord,
    end:   Coord,
}

impl CoordIter {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self {
            cur: start,
            start,
            end,
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.y < self.end.y {
            let c = self.cur;

            self.cur.x += 1;

            if self.cur.x >= self.end.x {
                self.cur.y += 1;
                self.cur.x = self.start.x;
            }

            Some(c)
        }
        else {
            None
        }
    }
}