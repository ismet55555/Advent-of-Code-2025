//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use log::{debug, error, info, warn};
use nom::Input;
use std::collections::HashSet;
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

/// Parse string input and return
/// fresh ingredient ID ranges
///
/// # Arguments
///
/// * `input` - String of raw input text
fn get_id_ranges_from_input(input: &str) -> Vec<(i64, i64)> {
    let split: Vec<&str> = input.split("\n\n").collect();
    split[0]
        .lines()
        .filter(|item| !item.is_empty())
        .map(|item| {
            let (min, max) = item.split_once("-").unwrap();
            (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap())
        })
        .collect()
}

/// Parse string input and return
/// ingredient IDs
///
/// # Arguments
///
/// * `input` - String of raw input text
fn get_ids_from_input(input: &str) -> Vec<i64> {
    let split: Vec<&str> = input.split("\n\n").collect();
    split[1]
        .lines()
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<i64>().unwrap())
        .collect()
}

/// Check if there is any overlap or adjacency between the two ranges
fn get_overlap_limits(
    (min_1, max_1): (i64, i64),
    (min_2, max_2): (i64, i64),
) -> (bool, (i64, i64)) {
    let has_overlap = if min_1 >= min_2 && max_1 <= max_2 {
        // Overlap - All first included into second
        debug!("OVERLAP - First included in second - ({min_2} -> {max_2})");
        true
    } else if min_1 <= min_2 && max_1 >= max_2 {
        // Overlap - All second included first
        debug!("OVERLAP - Second included in first - ({min_1} -> {max_1})");
        true
    } else if min_1 <= min_2 && max_1 >= min_2 {
        // Overlap - Shifted right
        debug!("OVERLAP - Second Shifted Right - ({min_1} -> {max_2})");
        true
    } else if min_1 >= min_2 && min_1 <= max_2 {
        // Overlap - Shifted left
        debug!("OVERLAP - Second Shifted Left - ({min_2} -> {max_1})");
        true
    } else if max_1 + 1 == min_2 {
        // Adjacent - First ends right before second starts
        debug!("ADJACENT - Second right after first - ({min_1} -> {max_2})");
        true
    } else if max_2 + 1 == min_1 {
        // Adjacent - Second ends right before first starts
        debug!("ADJACENT - First right after second - ({min_2} -> {max_1})");
        true
    } else {
        debug!("NO OVERLAP OR ADJACENCY");
        false
    };

    if has_overlap {
        let combined_values = [min_1, min_2, max_1, max_2];
        let min_out = combined_values.iter().min().unwrap();
        let max_out = combined_values.iter().max().unwrap();
        (true, (*min_out, *max_out))
    } else {
        (false, (0, 0))
    }
}

/// Consolidate all Overlaps
fn combine_any_overlap_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|(min, _)| *min);
    debug!("Sorted ranges: {:?}", ranges);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    let mut current = ranges[0];
    debug!("Starting with: ({}, {})", current.0, current.1);

    // Single pass through sorted ranges
    for &next in &ranges[1..] {
        debug!(
            "Current: ({} -> {})  |  Next: ({} -> {})",
            current.0, current.1, next.0, next.1
        );

        let (has_overlap, (min_new, max_new)) = get_overlap_limits(current, next);

        if has_overlap {
            // Extend current range
            current = (min_new, max_new);
            debug!("MERGED to: ({}, {})", current.0, current.1);
        } else {
            // Save current, start new range
            debug!("NO MERGE - Saving ({}, {})", current.0, current.1);
            merged.push(current);
            current = next;
        }
    }

    // Last Range
    merged.push(current);
    debug!("Final merged ranges: {:?}", merged);

    merged
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i32 {
    let fresh_id_ranges = get_id_ranges_from_input(&input);
    debug!("{fresh_id_ranges:?}");

    let ingredient_ids = get_ids_from_input(&input);
    debug!("{ingredient_ids:?}");

    let mut fresh_ingredient_count: i32 = 0;

    'id_loop: for id in &ingredient_ids {
        for (min, max) in &fresh_id_ranges {
            // Check if ID is within fresh range
            if id <= max && id >= min {
                fresh_ingredient_count += 1;

                // The ingredient is within at least one range, stop looking
                continue 'id_loop;
            }
        }
    }

    fresh_ingredient_count
}

/// Part 2 Calculations
fn part_2(input: String) -> i64 {
    let fresh_id_ranges = get_id_ranges_from_input(&input);
    debug!("Original ranges: {:?}", fresh_id_ranges);

    let merged_ranges = combine_any_overlap_ranges(fresh_id_ranges);
    debug!("Merged ranges: {:?}", merged_ranges);

    let total_count: i64 = merged_ranges.iter().map(|(min, max)| max - min + 1).sum();

    total_count
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
        let expected = 14;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
