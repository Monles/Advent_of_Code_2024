use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Function to calculate total distance between two sorted lists
fn calculate_total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Sort both lists
    left.sort();
    right.sort();

    // Compute total distance
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs()) // Pairwise absolute differences
        .sum()
}

fn main() -> io::Result<()> {
    // Path to the input file
    let path = "input.txt";

    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Vectors to store the left and right list numbers
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if numbers.len() == 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    // Ensure lists are of the same length
    if left_numbers.len() != right_numbers.len() {
        eprintln!("Error: Lists are not the same length!");
        return Ok(());
    }

    // Calculate total distance
    let total_distance = calculate_total_distance(left_numbers, right_numbers);

    // Print the total distance
    println!("Total Distance: {}", total_distance);

    Ok(())
}
