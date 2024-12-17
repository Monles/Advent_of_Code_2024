fn is_safe_report(report: &[i32]) -> bool {
    // Check if the report has fewer than 2 levels; such reports are trivially safe.
    if report.len() < 2 {
        return true;
    }

    // Flags to determine the trend
    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];

        if diff == 0 || diff < -3 || diff > 3 {
            // Invalid difference: zero or outside the range [1, 3]
            return false;
        }

        if diff > 0 {
            decreasing = false; // Upward difference invalidates decreasing
        } else if diff < 0 {
            increasing = false; // Downward difference invalidates increasing
        }
    }

    // Report is safe if it's strictly increasing or strictly decreasing
    increasing || decreasing
}

fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| is_safe_report(report)).count()
}

fn main() {
    // Example puzzle input: reports
    let reports = vec![
        vec![7, 6, 4, 2, 1], // Safe: decreasing by 1 or 2
        vec![1, 2, 7, 8, 9], // Unsafe: increase by 5
        vec![9, 7, 6, 2, 1], // Unsafe: decrease by 4
        vec![1, 3, 2, 4, 5], // Unsafe: mixed trend
        vec![8, 6, 4, 4, 1], // Unsafe: invalid 4->4
        vec![1, 3, 6, 7, 9], // Safe: increasing by 1, 2, or 3
    ];

    // Count the safe reports
    let safe_count = count_safe_reports(&reports);

    // Print the result
    println!("Number of safe reports: {}", safe_count);
}
