use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the input file path
    let path = "input.txt";

    // Open the file and handle errors
    let input_file = File::open(&Path::new(path))?;
    let reader = io::BufReader::new(input_file);

    // Define the regex pattern to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern");

    let mut total_sum = 0;

    // Process each line in the file
    for line in reader.lines() {
        if let Ok(line_content) = line {
            // Iterate through all matches of the regex in the line
            for caps in re.captures_iter(&line_content) {
                // Safely parse the two numbers and handle errors
                if let (Ok(num1), Ok(num2)) = (caps[1].parse::<i32>(), caps[2].parse::<i32>()) {
                    let product = num1 * num2;
                    total_sum += product;
                    println!("Valid mul({}, {}) -> {}", num1, num2, product);
                }
            }
        }
    }

    // Print the final result
    println!("The sum of all valid mul instructions is: {}", total_sum);

    Ok(())
}
