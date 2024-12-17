use regex::Regex;
    
fn main() {
    // Example of corrupted memory input
    let corrupted_memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    // Regular expression to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total_sum = 0;

    // Find all valid mul(X,Y) matches
    for caps in re.captures_iter(corrupted_memory) {
        // Extract the two numbers from the match
        let num1: i32 = caps[1].parse().unwrap();
        let num2: i32 = caps[2].parse().unwrap();

        // Calculate the product and add it to the total sum
        let product = num1 * num2;
        total_sum += product;

        println!("Valid mul({}, {}) -> {}", num1, num2, product);
    }

    // Print the final result
    println!("The sum of all valid mul instructions is: {}", total_sum);
}
