fn calculate_total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Step 1: Sort both input lists.
    left.sort();
    right.sort();

    // Step 2: Pair elements and calculate distances.
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn main() {
    // Input example: two lists of integers.
    let left_list = vec![3, 4, 2, 1, 3, 3];
    let right_list = vec![4, 3, 5, 3, 9, 3];

    // Calculate the total distance.
    let total_distance = calculate_total_distance(left_list, right_list);

    // Output the result.
    println!("The total distance is: {}", total_distance);
}
