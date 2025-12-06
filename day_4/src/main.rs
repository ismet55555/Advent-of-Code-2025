//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use log::{debug, error, info, warn};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    debug!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

fn load_grid_positions(input: &str) -> Vec<Vec<char>> {
    let out: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    out
}

#[derive(Debug)]
struct Grid {
    input: String,
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: String) -> Self {
        let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();
        Grid {
            input,
            grid,
            rows,
            cols,
        }
    }

    fn debug_print_grid(&self) {
        for row in self.grid.clone() {
            debug!("{:?}", row.into_iter().join(""));
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<char> {
        let steps: Vec<i32> = vec![-1, 0, 1];

        let mut neighbours: Vec<char> = Vec::new();

        for row_step in steps.clone() {
            for col_step in steps.clone() {
                // Do not check the provided position
                if row_step == 0 && col_step == 0 {
                    continue;
                }

                let row_index = row as i32 + row_step;
                let col_index = col as i32 + col_step;

                // Check rows boundaries
                if row_index < 0 {
                    continue;
                }
                if row_index > (self.rows - 1).try_into().unwrap() {
                    continue;
                }

                // Check column boundaries
                if col_index < 0 {
                    continue;
                }
                if col_index > (self.cols - 1).try_into().unwrap() {
                    continue;
                }

                let item = self.grid[row_index as usize][col_index as usize];
                neighbours.push(item);
            }
        }
        neighbours
    }

    /// Find number of neighbouring rolls and how many are accessible
    fn count_neighbours_rolls(&self, neighbours: Vec<char>) -> i32 {
        neighbours
            .iter()
            .filter(|item| **item == '@')
            .count()
            .try_into()
            .unwrap()
    }

    /// Find all accessible rolls in the grid
    fn find_accessible_rolls_in_grid(&self) -> Vec<(usize, usize)> {
        let mut accessible_rolls: Vec<(usize, usize)> = Vec::new();

        for row_index in 0..self.rows {
            for col_index in 0..self.cols {
                if self.grid[row_index][col_index] == '.' {
                    continue;
                }

                let neighbours = self.get_neighbours(row_index, col_index);
                let roll_count = self.count_neighbours_rolls(neighbours);

                if roll_count > 3 {
                    continue;
                }

                accessible_rolls.push((row_index, col_index));
            }
        }
        accessible_rolls
    }

    /// Replace accessible rolls with '.' characters in grid
    fn remove_accessible_rolls(&mut self, accessible_rolls: Vec<(usize, usize)>) {
        for (row_index, col_index) in accessible_rolls.iter() {
            self.grid[*row_index][*col_index] = '.';
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let grid = Grid::new(input);
    grid.find_accessible_rolls_in_grid().len() as i32
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let mut grid = Grid::new(input);

    let mut rolls_removed: i32 = 0;

    grid.debug_print_grid();
    let mut accessible_rolls = grid.find_accessible_rolls_in_grid();
    let mut accessible_roll_count = accessible_rolls.len();
    rolls_removed += accessible_roll_count as i32;

    while accessible_roll_count > 0 {
        debug!("-------------------------------------------------");
        grid.debug_print_grid();
        grid.remove_accessible_rolls(accessible_rolls);
        accessible_rolls = grid.find_accessible_rolls_in_grid();
        accessible_roll_count = accessible_rolls.len();
        rolls_removed += accessible_roll_count as i32;
    }

    rolls_removed
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    // Initialize the logger at the start of main
    env_logger::init();

    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("Part 1: {}\n\n", part_1_output);

    let part_2_input = load_text_file("inputs/input_part2.txt");
    let part_2_output = part_2(part_2_input);
    println!("Part 2: {}", part_2_output);

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////
//                            TESTS
//////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = load_text_file("inputs/sample_part1.txt");
        let expected = 13;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 43;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
