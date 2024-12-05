use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Grid {
    height: usize,
    width: usize,
    pub data: Vec<Vec<char>>,
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
enum Direction {
    All,
    Left,
    TopLeft,
    Top,
    Right,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
}

impl Grid {
    pub fn new(height: usize, width: usize, data: Vec<String>) -> Result<Grid, String> {
        if height != data.len() || width != data[0].len() {
            return Err(format!("Data size does not match with the grid dimensions. Expected ({}, {}) but got ({}, {})", data.len(), data[0].len(), height, width));
        }
        Ok(Grid {
            height,
            width,
            data: data.iter().map(|s| s.chars().collect()).collect(),
        })
    }

    // FOR PART 1

    pub fn count_word(&self, word: String) -> usize {
        let mut count: usize = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == word.chars().nth(0).unwrap() {
                    count += self.count_words_starting_there(i, j, &word[1..], Direction::All);
                }
            }
        }
        count
    }

    fn count_words_starting_there(
        &self,
        i: usize,
        j: usize,
        word: &str,
        direction: Direction,
    ) -> usize {
        if direction == Direction::All {
            let mut count: usize = 0;
            for dir in Direction::iter() {
                if dir == Direction::All {
                    continue;
                }
                count += self.count_words_starting_there(i, j, word, dir);
            }
            return count;
        }
        let (a, b) = self.map_indices_to_check(i as i32, j as i32, &direction);
        if !self.are_valid(a, b) {
            return 0;
        }
        if word.chars().nth(0).unwrap() == self.data[a as usize][b as usize] {
            if word.len() == 1 {
                return 1;
            }
            return self.count_words_starting_there(a as usize, b as usize, &word[1..], direction);
        }
        0
    }

    // FOR PART 2

    pub fn count_crosses(&self, _word: String) -> usize {
        let mut count: usize = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                // Sorry I hardcoded it lol
                if self.data[i][j] == 'A' {
                    count += self.check_if_cross(i, j);
                }
            }
        }
        count
    }

    fn check_if_cross(&self, i: usize, j: usize) -> usize {
        let top_left = self.data_contains(i, j, Direction::TopLeft);
        let top_right = self.data_contains(i, j, Direction::TopRight);
        let bottom_left = self.data_contains(i, j, Direction::BottomLeft);
        let bottom_right = self.data_contains(i, j, Direction::BottomRight);
        let mut chars = [top_left, top_right, bottom_left, bottom_right];
        chars.sort();
        if chars == ['M', 'M', 'S', 'S'] && top_right != bottom_left && top_left != bottom_right {
            return 1;
        }
        0
    }

    fn data_contains(&self, i: usize, j: usize, direction: Direction) -> char {
        let (a, b) = self.map_indices_to_check(i as i32, j as i32, &direction);
        if self.are_valid(a, b) {
            return self.data[a as usize][b as usize];
        }
        return '0';
    }

    fn are_valid(&self, a: i32, b: i32) -> bool {
        a >= 0 && b >= 0 && (a as usize) < self.height && (b as usize) < self.width
    }

    fn map_indices_to_check(&self, i: i32, j: i32, direction: &Direction) -> (i32, i32) {
        let a: i32 = match direction {
            Direction::Top | Direction::TopLeft | Direction::TopRight => i - 1,
            Direction::Right | Direction::Left => i,
            Direction::Bottom | Direction::BottomLeft | Direction::BottomRight => i + 1,
            _ => -1,
        };
        let b: i32 = match direction {
            Direction::BottomLeft | Direction::Left | Direction::TopLeft => j - 1,
            Direction::Top | Direction::Bottom => j,
            Direction::TopRight | Direction::Right | Direction::BottomRight => j + 1,
            _ => -1,
        };
        (a, b)
    }
}
