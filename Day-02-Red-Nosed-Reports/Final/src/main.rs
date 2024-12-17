use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to check if a single report is safe
fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; // Trivial case: fewer than 2 levels
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff < -3 || diff > 3 {
            // Invalid difference: zero or outside the range [-3, 3]
            return false;
        }

        if diff > 0 {
            decreasing = false; // If increasing, not strictly decreasing
        } else if diff < 0 {
            increasing = false; // If decreasing, not strictly increasing
        }
    }

    increasing || decreasing // Safe if strictly increasing or decreasing
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input_file = File::open(&Path::new(path))?;
    let reader = io::BufReader::new(input_file);

    let mut safe_count = 0;
    let mut total_reports = 0;

    for line in reader.lines() {
        if let Ok(report_line) = line {
            // Parse the line into a vector of integers
            let report: Vec<i32> = report_line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if !report.is_empty() {
                total_reports += 1;
                if is_safe_report(&report) {
                    safe_count += 1;
                }
            }
        }
    }

    println!("Total reports: {}", total_reports);
    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
