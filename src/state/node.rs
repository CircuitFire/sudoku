use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct FoundData {
    pub guess_level: usize,
    pub guess_id:    Option<usize>,
    pub num:         usize,
    pub conflicts:   Vec<Coord>,
}

#[derive(Clone, Debug)]
pub enum Node {
    Found(FoundData),
    Possible(Vec<bool>),
}

impl Node {
    pub fn new(size: usize) -> Self {
        Self::Possible(vec![true; size])
    }

    pub fn solution(&self) -> Option<usize> {
        if let Self::Found(data) = self {
            Some(data.num)
        }
        else {
            None
        }
    }

    pub fn possibilities(&self) -> Option<& Vec<bool>> {
        if let Self::Possible(list) = self {
            Some(&list)
        }
        else {
            None
        }
    }

    pub fn num_possibilities(&self) -> Option<usize> {
        if let Self::Possible(list) = self {
            Some(list.iter().filter(|x| **x).count())
        }
        else {
            None
        }
    }

    pub fn is_found(&self) -> bool {
        if let Self::Found(_) = self {
            true
        }
        else {
            false
        }
    }

    pub fn get_num(&self) -> Option<usize> {
        if let Self::Found(data) = self {
            Some(data.num)
        }
        else {
            None
        }
    }

    pub fn add_conflict(&mut self, conflict: Coord) {
        if let Self::Found(ref mut data) = self {
            data.conflicts.push(conflict)
        }
    }

    pub fn remove_conflict(&mut self, conflict: Coord) {
        if let Self::Found(ref mut data) = self {
            for i in 0..data.conflicts.len() {
                if data.conflicts[i] == conflict {
                    data.conflicts.remove(i);
                    break;
                }
            }
        }
    }

    pub fn conflicts(&self) -> Option<&Vec<Coord>> {
        if let Self::Found(ref data) = self {
            Some(&data.conflicts)
        }
        else {
            None
        }
    }

    pub fn has_conflicts(&self) -> bool {
        if let Self::Found(ref data) = self {
            !data.conflicts.is_empty()
        }
        else {
            false
        }
    }

    pub fn has_possibilities(&self) -> bool {
        if let Self::Possible(ref list) = self {
            list.iter().filter(|x| **x).count() > 0
        }
        else {
            true
        }
    }

    pub fn has_problems(&self) -> bool {
        match self {
            Self::Found(ref data) => {
                !data.conflicts.is_empty()
            }
            Self::Possible(ref list) => {
                list.iter().filter(|x| **x).count() == 0
            }
        }
    }

    pub fn reset_possible(&mut self) {
        if let Self::Possible(ref mut list) = self {
            list.fill(true);
        }
    }

    pub fn set(&mut self, num: usize, guess_level: usize) {
        *self = Self::Found(FoundData{
            guess_level,
            guess_id: None,
            num,
            conflicts: Vec::new(),
        });
    }

    pub fn set_guess_id(&mut self, id: usize) {
        if let Self::Found(ref mut data) = self {
            data.guess_id = Some(id);
        }
    }

    pub fn get_guess_id(&mut self) -> Option<usize>{
        if let Self::Found(ref mut data) = self {
            data.guess_id
        }
        else {
            None
        }
    }

    pub fn reset(&mut self, size: usize) {
        *self = Self::Possible(vec![true; size]);
    }

    pub fn count(&self) -> Option<usize> {
        match self {
            Self::Possible(ref list) => {
                Some(list.iter().filter(|x| **x).count())
            }
            Self::Found{..} => {
                None
            }
        }
    }

    pub fn guess_reset(&mut self, cur_guess: usize) -> bool {
        if let Self::Found(ref data) = self {
            if data.guess_level > cur_guess {
                return true
            }
        }

        false
    }

    pub fn try_solve(&mut self, guess: usize) -> Option<usize> {
        if let Self::Possible(ref list) = self {
            let mut found = None;

            for (i, x) in list.iter().enumerate() {
                if *x {
                    if found.is_none() {
                        found = Some(i);
                    }
                    else {
                        return None
                    }
                }
            }

            if let Some(num) = found {
                self.set(num, guess);

                return Some(num)
            }
        }
        None
    }
}