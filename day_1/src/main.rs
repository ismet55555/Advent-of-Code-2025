//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}
//////////////////////////////////////////////////////////////////////////////

/// Get dial directions and clicks
fn get_directions_and_clicks(input: &str) -> (Vec<String>, Vec<i32>) {
    // Get the movement/spin direction letters
    let directions: Vec<String> = input
        .lines()
        .map(|line| line.chars().take_while(|c| c.is_alphabetic()).collect())
        .collect();

    // Get the dial clicks numbers
    let clicks: Vec<i32> = input
        .lines()
        .map(|line| {
            line.chars()
                .skip_while(|c| c.is_alphabetic())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .collect();

    (directions, clicks)
}

/// Get the cyclic value from index
/// If out of range, it will roll over
/// Also checks if index wrapped around
fn cyclic_index(vec: &[i32], index: i32) -> (&i32, bool) {
    let wrapped = index.rem_euclid(vec.len() as i32) as usize;
    let did_wrap = index < 0 || index >= vec.len() as i32;
    (&vec[wrapped], did_wrap)
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> u32 {
    let (directions, clicks) = get_directions_and_clicks(&input);

    let lock_numbers: Vec<i32> = (0..=99).collect();
    let mut zero_counter: u32 = 0;
    let mut current_index: i32 = 50;

    // Loop through all instructions
    for (direction, click) in zip(directions, clicks) {
        // println!("-------------------------------");
        // println!("Direction: {:?}", direction);
        // println!("Click: {:?}", click);
        // println!("-------------------------------");

        if direction == "R" {
            // Add to current index (clicks)
            current_index += click;
        } else {
            // Subtract from current index (clicks)
            current_index -= click;
        }

        // println!("Current index: {}", current_index);
        let (dial_position, _) = cyclic_index(&lock_numbers, current_index);
        if *dial_position == 0 {
            zero_counter += 1;
        }
    }

    zero_counter
}

/// Part 2 Calculations
fn part_2(input: String) -> i32 {
    let (directions, clicks) = get_directions_and_clicks(&input);

    let lock_numbers: Vec<i32> = (0..=99).collect();
    let mut zero_counter: i32 = 0; // Crossing or ending up at zero
    let mut current_index: i32 = 50;
    let mut previous_index: i32;

    // Loop through all instructions
    for (direction, click) in zip(directions, clicks) {
        // println!("-------------------------------");
        // println!("Direction: {:?}", direction);
        // println!("Clicks: {:?}", click);
        // println!("-------------------------------");

        previous_index = current_index;

        if direction == "R" {
            // Add to current index (clicks)
            current_index += click;
        } else {
            // Subtract from current index (clicks)
            current_index -= click;
        }

        // Count how many times we pass through or land on a multiple of 100
        // (multiples of 100 are where the dial points to 0)
        let crosses = if previous_index < current_index {
            // Moving right: count multiples in (previous_index, current_index]
            current_index.div_euclid(100) - previous_index.div_euclid(100)
        } else if previous_index > current_index {
            // Moving left: count multiples in [current_index, previous_index)
            (previous_index - 1).div_euclid(100) - (current_index - 1).div_euclid(100)
        } else {
            0 // No movement
        };

        zero_counter += crosses;
    }

    zero_counter
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
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
        let expected = 3;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 6;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
