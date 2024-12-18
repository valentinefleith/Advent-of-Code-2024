use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    height: usize,
    width: usize,
    pub data: Vec<Vec<char>>,
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

    fn track_antennas(&self) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j].is_alphanumeric() {
                    antennas
                        .entry(self.data[i][j])
                        .or_insert_with(Vec::new)
                        .push((i, j));
                }
            }
        }
        antennas
    }

    fn is_valid(&self, combination: (i32, i32)) -> bool {
        combination.0 >= 0
            && combination.1 >= 0
            && (combination.0 as usize) < self.height
            && (combination.1 as usize) < self.width
    }

    fn find_current_antinodes(&self, coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut antinodes = vec![];
        for (coord1, coord2) in coordinates.iter().tuple_combinations() {
            let (dy, dx) = (
                coord1.0 as i32 - coord2.0 as i32,
                coord1.1 as i32 - coord2.1 as i32,
            );
            let comb = (coord1.0 as i32 + dy, coord1.1 as i32 + dx);
            if self.is_valid(comb) {
                antinodes.push((comb.0 as usize, comb.1 as usize));
            }
            let (dy, dx) = (
                coord2.0 as i32 - coord1.0 as i32,
                coord2.1 as i32 - coord1.1 as i32,
            );
            let comb = (coord2.0 as i32 + dy, coord2.1 as i32 + dx);
            if self.is_valid(comb) {
                antinodes.push((comb.0 as usize, comb.1 as usize));
            }
        }
        antinodes
    }

    fn get_combination(coord1: &(i32, i32), coord2: &(i32, i32)) -> (i32, i32) {
        let (dy, dx) = (
            coord1.0 as i32 - coord2.0 as i32,
            coord1.1 as i32 - coord2.1 as i32,
        );
        (coord1.0 as i32 + dy, coord1.1 as i32 + dx)
    }

    fn find_antinodes_rec(
        &self,
        coord1: (usize, usize),
        coord2: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut antinodes = vec![coord1, coord2];
        let comb1 = Map::get_combination(
            &(coord1.0 as i32, coord1.1 as i32),
            &(coord2.0 as i32, coord2.1 as i32),
        );
        if self.is_valid(comb1) {
            antinodes.extend(self.find_antinodes_rec((comb1.0 as usize, comb1.1 as usize), coord1));
        }
        antinodes
    }

    fn find_antinodes_p2(&self, coordinates: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut antinodes = vec![];
        for (coord1, coord2) in coordinates.iter().tuple_combinations() {
            antinodes.extend(self.find_antinodes_rec(*coord1, *coord2));
            antinodes.extend(self.find_antinodes_rec(*coord2, *coord1));
        }
        antinodes
    }

    fn find_antinode_locations(
        &self,
        antennas: HashMap<char, Vec<(usize, usize)>>,
        part1: bool,
    ) -> HashSet<(usize, usize)> {
        let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new();
        for (_, coordinates) in &antennas {
            let current_antinodes: Vec<(usize, usize)> = if part1 {
                self.find_current_antinodes(coordinates)
            } else {
                self.find_antinodes_p2(coordinates)
            };
            for antinode in current_antinodes {
                unique_antinodes.insert(antinode);
            }
        }
        unique_antinodes
    }

    pub fn count_antinodes(&self, part1: bool) -> usize {
        let antennas: HashMap<char, Vec<(usize, usize)>> = self.track_antennas();
        let unique_antinodes: HashSet<(usize, usize)> =
            self.find_antinode_locations(antennas, part1);
        unique_antinodes.len()
    }
}
