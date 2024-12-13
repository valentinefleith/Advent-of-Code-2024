use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Direction {
    fn from(&self, current_pos: Pos) -> Pos {
        match self {
            Direction::Left => Pos {
                x: current_pos.x - 1,
                y: current_pos.y,
            },
            Direction::Top => Pos {
                x: current_pos.x,
                y: current_pos.y - 1,
            },
            Direction::Right => Pos {
                x: current_pos.x + 1,
                y: current_pos.y,
            },
            Direction::Bottom => Pos {
                x: current_pos.x,
                y: current_pos.y + 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Map {
    height: usize,
    width: usize,
    data: Vec<Vec<u32>>,
}

impl Map {
    pub fn new(height: usize, width: usize, data: Vec<String>) -> Result<Map, String> {
        if height != data.len() || width != data[0].len() {
            return Err(format!("Data size does not match with the grid dimensions. Expected ({}, {}) but got ({}, {})", data.len(), data[0].len(), height, width));
        }
        Ok(Map {
            height,
            width,
            data: data
                .iter()
                .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        })
    }

    pub fn get_all_trailheads(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == 0 {
                    count += self.get_trailhead_score_p2(
                        Pos {
                            x: (j as i32),
                            y: (i as i32),
                        },
                        1,
                    );
                }
            }
        }
        count as i32
    }
    fn get_trailhead_score_p1(&self, position: Pos, looking_for: u32) -> HashSet<Pos> {
        if looking_for == 10 {
            let mut victory_pos = HashSet::new();
            victory_pos.insert(position);
            return victory_pos;
        }
        let mut arrivals = HashSet::new();
        for direction in Direction::iter() {
            let next_pos: Pos = direction.from(position);
            if !self.is_valid(next_pos) {
                continue;
            }
            if self.data[next_pos.y as usize][next_pos.x as usize] == looking_for {
                arrivals.extend(self.get_trailhead_score_p1(next_pos, looking_for + 1));
            }
        }
        arrivals
    }

    fn get_trailhead_score_p2(&self, position: Pos, looking_for: u32) -> i32 {
        if looking_for == 10 {
            return 1;
        }
        let mut count = 0;
        for direction in Direction::iter() {
            let next_pos: Pos = direction.from(position);
            if !self.is_valid(next_pos) {
                continue;
            }
            if self.data[next_pos.y as usize][next_pos.x as usize] == looking_for {
                count += self.get_trailhead_score_p2(next_pos, looking_for + 1);
            }
        }
        count
    }

    fn is_valid(&self, position: Pos) -> bool {
        position.x >= 0
            && position.y >= 0
            && (position.x as usize) < self.width
            && (position.y as usize) < self.height
    }
}
