use indicatif::ProgressIterator;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Obstruction,
    Nothing,
    GuardStart,
}

impl Element {
    pub fn as_char(&self) -> char {
        match self {
            Element::Obstruction => '#',
            Element::Nothing => '.',
            Element::GuardStart => '^',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug)]
pub struct Map {
    height: usize,
    width: usize,
    data: Vec<Vec<char>>,
}

impl Map {
    pub fn new(height: usize, width: usize, data: Vec<String>) -> Result<Map, String> {
        if height != data.len() || width != data[0].len() {
            return Err(format!("Data size does not match with the grid dimensions. Expected ({}, {}) but got ({}, {})", data.len(), data[0].len(), height, width));
        }
        Ok(Map {
            height,
            width,
            data: data.iter().map(|s| s.chars().collect()).collect(),
        })
    }

    fn find_guard_starting_pos(&self) -> Option<(i32, i32)> {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == Element::GuardStart.as_char() {
                    return Some((i.try_into().unwrap(), j.try_into().unwrap()));
                }
            }
        }
        None
    }

    fn is_valid_pos(&self, a: i32, b: i32) -> bool {
        a >= 0 && b >= 0 && (a as usize) < self.height && (b as usize) < self.width
    }

    fn map_indices_to_pos(&self, i: i32, j: i32, direction: &Direction) -> (i32, i32) {
        let a: i32 = match direction {
            Direction::Top => i - 1,
            Direction::Right | Direction::Left => i,
            Direction::Bottom => i + 1,
        };
        let b: i32 = match direction {
            Direction::Left => j - 1,
            Direction::Top | Direction::Bottom => j,
            Direction::Right => j + 1,
        };
        (a, b)
    }

    fn get_next_direction(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }

    pub fn find_route(&self) -> HashSet<(i32, i32)> {
        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        let (mut i, mut j) = self.find_guard_starting_pos().unwrap();
        positions.insert((i, j));
        let mut current_dir = Direction::Top;
        while self.is_valid_pos(i, j) {
            let (candidate_i, candidate_j) = self.map_indices_to_pos(i, j, &current_dir);
            if !self.is_valid_pos(candidate_i, candidate_j) {
                break;
            }
            if self.data[candidate_i as usize][candidate_j as usize]
                == Element::Obstruction.as_char()
            {
                current_dir = self.get_next_direction(&current_dir);
                continue;
            }
            (i, j) = (candidate_i, candidate_j);
            positions.insert((i, j));
        }
        positions
    }

    pub fn count_loops(&mut self, guard_positions: HashSet<(i32, i32)>) -> usize {
        let mut count: usize = 0;
        let starting_pos: (i32, i32) = self.find_guard_starting_pos().unwrap();
        for pos in guard_positions.into_iter().progress() {
            if self.data[pos.0 as usize][pos.1 as usize] != Element::Nothing.as_char() {
                continue;
            }
            self.data[pos.0 as usize][pos.1 as usize] = Element::Obstruction.as_char();
            if self.is_looping(starting_pos) {
                count += 1;
            }
            self.data[pos.0 as usize][pos.1 as usize] = Element::Nothing.as_char();
        }
        count
    }

    fn is_looping(&self, starting_pos: (i32, i32)) -> bool {
        let mut positions: HashSet<((i32, i32), Direction)> = HashSet::new();
        let (mut i, mut j) = starting_pos;
        let mut current_dir = Direction::Top;
        positions.insert(((i, j), current_dir));
        while self.is_valid_pos(i, j) {
            let (candidate_i, candidate_j) = self.map_indices_to_pos(i, j, &current_dir);
            if !self.is_valid_pos(candidate_i, candidate_j) {
                break;
            }
            if self.data[candidate_i as usize][candidate_j as usize]
                == Element::Obstruction.as_char()
            {
                current_dir = self.get_next_direction(&current_dir);
                continue;
            }
            (i, j) = (candidate_i, candidate_j);
            if positions.contains(&((i, j), current_dir)) {
                return true;
            }
            positions.insert(((i, j), current_dir));
        }
        false
    }
}
