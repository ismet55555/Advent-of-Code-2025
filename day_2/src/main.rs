//! Advent of code day

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////////////

fn load_text_file(filepath: &str) -> String {
    println!("Loading file: {} ...", filepath);
    let contents: String = fs::read_to_string(filepath).expect("ERROR: Failed to read input file");

    contents
}

fn parse_to_number_ranges(input: &str) -> Vec<(i64, i64)> {
    let items: Vec<_> = input.split(",").map(|n| n.trim()).collect();

    let out: Vec<(i64, i64)> = items
        .iter()
        .map(|item| {
            let parts: Vec<&str> = item.split("-").collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect();

    out
}

fn find_pattern(s: &str) -> Option<&str> {
    let len = s.len();
    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            if pattern.repeat(len / pattern_len) == s {
                return Some(pattern);
            }
        }
    }
    None
}

//////////////////////////////////////////////////////////////////////////////

/// Part 1 Calculations
fn part_1(input: String) -> i64 {
    // Split up items
    let number_ranges = parse_to_number_ranges(&input);

    let mut invalid_id_sum: i64 = 0;
    let mut invalid_id_count: i64 = 0;

    // Loop over all range pairs
    for (start, end) in number_ranges {
        // Create a vector range from boudry values
        let range: Vec<i64> = (start..=end).collect();

        let mut number_string: String;

        for number in range {
            // convert to string
            number_string = number.to_string();

            // Skip numbers that have odd number of characters
            if number_string.len() % 2 == 1 {
                continue;
            }

            // Check if first half is the same as second half of the number string
            // Invalid number
            let mid = number_string.len() / 2;
            let (first_half, second_half) = number_string.split_at(mid);
            if first_half == second_half {
                invalid_id_count += 1;
                invalid_id_sum += number;
            }
        }
    }

    invalid_id_sum
}

/// Part 2 Calculations
fn part_2(input: String) -> i64 {
    // Split up items
    let number_ranges = parse_to_number_ranges(&input);

    let mut invalid_id_sum: i64 = 0;
    let mut invalid_id_count: i64 = 0;

    // Loop over all range pairs
    for (start, end) in number_ranges {
        let (count, sum) = (start..=end)
            .filter(|&number| {
                let number_string = number.to_string();
                // Check if the number has a repeating pattern
                find_pattern(&number_string).is_some()
            })
            .fold((0i64, 0i64), |(count, sum), number| {
                (count + 1, sum + number)
            });

        invalid_id_count += count;
        invalid_id_sum += sum;
    }

    invalid_id_sum
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
        let expected = 1227775554;
        let actual = part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let input = load_text_file("inputs/sample_part2.txt");
        let expected = 4174379265;
        let actual = part_2(input);
        assert_eq!(expected, actual);
    }
}
