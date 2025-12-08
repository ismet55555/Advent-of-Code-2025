//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use itertools::Itertools;
use log::{debug, error, info, warn};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

/// Load the raw text file into a string
///
/// # Arguments
///
/// * `filepath` - File to load into a string
fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

/// Parse Input vertically for each column
/// Treating whitespace all the same
fn parse_input_per_column(input: &str) -> Vec<(i64, i64, i64, i64, char)> {
    let lines: Vec<_> = input.lines().collect();

    let numbers_1: Vec<i64> = lines[0]
        .split_whitespace()
        .map(|item| item.parse::<i64>().unwrap())
        .collect();

    let numbers_2: Vec<i64> = lines[1]
        .split_whitespace()
        .map(|item| item.parse::<i64>().unwrap())
        .collect();

    let numbers_3: Vec<i64> = lines[2]
        .split_whitespace()
        .map(|item| item.parse::<i64>().unwrap())
        .collect();

    let numbers_4: Vec<i64> = lines[3]
        .split_whitespace()
        .map(|item| item.parse::<i64>().unwrap())
        .collect();

    let operations: Vec<char> = lines[4]
        .split_whitespace()
        // .inspect(|x| println!("Value: {:?}", x))
        .map(|item| item.parse::<char>().unwrap())
        .collect();

    let mut out: Vec<(i64, i64, i64, i64, char)> = Vec::new();
    for index in 0..numbers_1.len() {
        out.push((
            numbers_1[index],
            numbers_2[index],
            numbers_3[index],
            numbers_4[index],
            operations[index],
        ));
    }

    out
}

/// Split input into number rows and operation row
fn parse_divide_nums_ops(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let input_rows = input.lines().count();

    // Parse numbers
    let nums: Vec<Vec<char>> = input
        .lines()
        .take(input_rows - 1)
        .map(|line| line.chars().collect())
        .collect();
    // debug!("{:?}", nums);

    // Parse math operations
    let ops: Vec<char> = input.lines().last().unwrap().chars().collect();
    // debug!("{:?}", ops);

    (nums, ops)
}

/// Check if all rows contain whitespace at given column index
fn is_all_whitespace_at_index(char_vecs: &Vec<Vec<char>>, index: usize) -> bool {
    let mut nums_col: Vec<char> = Vec::new();

    for char_vec in char_vecs {
        nums_col.push(char_vec[index]);
    }

    nums_col.iter().filter(|item| !item.is_whitespace()).count() == 0
}

/// Group aligned multi-digit numbers with their corresponding operations
fn get_aligned_column_numbers_for_operation(
    num_rows: Vec<Vec<char>>,
    ops_row: Vec<char>,
) -> (Vec<Vec<i64>>, Vec<char>) {
    // Keeps up the index of the aligned operation / number column
    let mut op_count: usize = 0;

    // Vector to save all operations and parsed numbers
    let mut ops: Vec<char> = Vec::new();
    let mut nums: Vec<Vec<i64>> = Vec::new();

    // Loop operation positions
    for (op_index, op) in ops_row.iter().enumerate() {
        // Skip columns where operation line is blank/space
        if op.is_whitespace() {
            continue;
        }

        // Check if column is all number rows are blank/space for this starting index
        let mut is_all_whitespace = is_all_whitespace_at_index(&num_rows, op_index);

        let mut colum_nums_for_current_op: Vec<i64> = Vec::new();
        let mut sub_index: usize = op_index;

        while !is_all_whitespace && sub_index < ops_row.len() {
            let mut column_nums_current: Vec<char> = Vec::new();

            // Get all numbers for this current column/index
            for num_row in &num_rows {
                column_nums_current.push(num_row[sub_index]);
            }

            is_all_whitespace = is_all_whitespace_at_index(&num_rows, sub_index);
            // debug!("Is Blank: {}", is_all_whitespace);

            if !is_all_whitespace {
                // If at least one number in column, join and parse to i64
                let num_current: i64 = column_nums_current.iter().join("").trim().parse().unwrap();
                // debug!("Joined: {}", num_current);
                colum_nums_for_current_op.push(num_current);
            }

            sub_index += 1;
        }

        // Save the current numbers and op
        ops.push(*op);
        nums.push(colum_nums_for_current_op);

        op_count += 1;
    }

    debug!("{:?}", nums);
    debug!("{:?}", ops);

    (nums, ops)
}

/// Execute operations on grouped numbers and return sum
fn execute_operations(nums: Vec<Vec<i64>>, ops: Vec<char>) -> i64 {
    let out: i64 = nums
        .iter()
        .zip(ops)
        .map(|(num, op)| {
            if op == '*' {
                num.iter().product()
            } else if op == '+' {
                num.iter().sum()
            } else {
                0
            }
        })
        .sum();
    out
}

//////////////////////////////////////////////////////////////////////////////

/// Part 2 Calculations
fn part_2(input: String) -> i64 {
    // Simply parse, divide, and group each row
    let (nums, ops) = parse_divide_nums_ops(&input);

    // Group aligned numbers for each operation
    let (nums, ops) = get_aligned_column_numbers_for_operation(nums, ops);

    execute_operations(nums, ops)
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i64 {
    let input_parsed = parse_input_per_column(&input);

    let mut result: i64 = 0;

    // Loop over each vertical column from input
    for (num_1, num_2, num_3, num_4, op) in input_parsed {
        result += if op == '+' {
            num_1 + num_2 + num_3 + num_4
        } else if op == '*' {
            num_1 * num_2 * num_3 * num_4
        } else {
            0
        };
    }

    result
}

//////////////////////////////////////////////////////////////////////////////

/// Main
fn main() -> Result<()> {
    // Initialize the logger at the start of main - plain logs
    env_logger::builder()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "{}", record.args())
        })
        .init();

    let part_1_input = load_text_file("inputs/input_part1.txt");
    let part_1_output = part_1(part_1_input);
    println!("\nPart 1: {}\n\n", part_1_output);

    let part_2_input = load_text_file("inputs/input_part2.txt");
    let part_2_output = part_2(part_2_input);
    println!("\nPart 2: {}", part_2_output);

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
        let expected = 4277556;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 3263827;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
