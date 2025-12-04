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
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

// Return a vector of vectors
// Parse each line to be a vector of i32 numbers
fn parse_new_lines_into_vectors(input: String) -> Vec<Vec<i32>> {
    let out: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect()
        })
        .collect();

    out
}

/// Find the largest number in given vector and its index
fn find_largest_and_index(vec: &[i32]) -> Option<(usize, i32)> {
    vec.iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .map(|(index, &value)| (index, value))
}

/// Combine i32 integers as string then convert back to i32
/// Two integers
fn concatenate_i32(value_1: i32, value_2: i32) -> i32 {
    (value_1.to_string() + &value_2.to_string())
        .parse::<i32>()
        .unwrap()
}

/// Combine i32 integers as string then convert back to i32
/// Vector of integers
fn concatenate_i32_vec(vec: &[i32]) -> i32 {
    vec.iter()
        .map(|number| number.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let inputs: Vec<Vec<i32>> = parse_new_lines_into_vectors(input);
    let mut joltage_sum = 0;

    for bat_bank in inputs {
        debug!("======================================");
        debug!("Bank: {:?}", bat_bank);
        debug!("======================================");

        let mut max_joltage = 0;

        // Try all possible pairs (i, j) where i < j
        for i in 0..bat_bank.len() {
            for j in (i + 1)..bat_bank.len() {
                let joltage = concatenate_i32(bat_bank[i], bat_bank[j]);
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        debug!("Max Joltage: {}", max_joltage);
        joltage_sum += max_joltage;
    }

    joltage_sum
}

/// Part 2 Calculations
fn part_2(input: String) -> i64 {
    let inputs: Vec<Vec<i32>> = parse_new_lines_into_vectors(input);
    let mut joltage_sum = 0;
    for bat_bank in inputs {
        debug!("======================================");
        debug!("    Bank: {:?}", bat_bank);
        debug!("======================================");

        let mut digits: Vec<i32> = Vec::new();
        let mut max_joltage = 0;

        // Loop through each position
        for i in 0..bat_bank.len() {
            // println!(" ");
            println!("--------- Index: {} - Value: {} ---------", i, &bat_bank[i]);

            // Loop through each position again
            for i in 0..bat_bank.len() {
                // Need 1. Enough digits after, and 2. Largest

                let current_digit = &bat_bank[i];
                let remaining_digits = &bat_bank[i + 1..];

                println!("Current digit: {}", current_digit);
                println!("After digit:   {:?}", remaining_digits);

                // Check if enough digits after current digit
                if remaining_digits.len() < 12 {
                    println!("** SKIP - Not enough digits **");
                    continue;
                }
            }
        }
    }

    22
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    // Initialize the logger at the start of main
    env_logger::init();

    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("Part 1: {}\n\n", part_1_output);

    let part_2_input = load_text_file("inputs/sample_part2.txt");
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
        let expected = 357;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected: i64 = 3121910778619;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
