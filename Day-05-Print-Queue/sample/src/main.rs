use std::collections::{HashMap, HashSet};

fn main() {
    // Input rules and updates (replace with actual input as needed)
    let rules_input = vec![
        "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29", "53|29",
        "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61", "47|29", "75|13",
    ];
    let updates_input = vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
        vec![75, 97, 47, 61, 53],
        vec![61, 13, 29],
        vec![97, 13, 75, 29, 47],
    ];

    // Parse the rules into a map
    let rules = parse_rules(&rules_input);

    // Process each update and determine the middle page of correctly-ordered updates
    let mut sum_of_middle_pages = 0;
    for update in &updates_input {
        if is_update_valid(update, &rules) {
            sum_of_middle_pages += find_middle_page(update);
        }
    }

    // Print the result
    println!("Sum of middle pages: {}", sum_of_middle_pages);
}

fn parse_rules(rules_input: &[&str]) -> HashMap<i32, HashSet<i32>> {
    let mut rules = HashMap::new();

    for rule in rules_input {
        let parts: Vec<i32> = rule.split('|')
                                  .map(|x| x.parse::<i32>().unwrap())
                                  .collect();
        let (x, y) = (parts[0], parts[1]);

        rules.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    rules
}

fn is_update_valid(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let position: HashMap<i32, usize> = update.iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    for (&x, dependents) in rules {
        if let Some(&x_pos) = position.get(&x) {
            for &y in dependents {
                if let Some(&y_pos) = position.get(&y) {
                    if x_pos >= y_pos {
                        return false; // x must come before y
                    }
                }
            }
        }
    }

    true
}

fn find_middle_page(update: &[i32]) -> i32 {
    let mid_index = update.len() / 2;
    update[mid_index]
}
