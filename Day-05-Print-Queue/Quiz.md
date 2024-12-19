use std::collections::HashMap;

fn is_valid_order(update: &[usize], rules: &HashMap<(usize, usize), bool>) -> bool {
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            if let Some(&must_be_before) = rules.get(&(update[i], update[j])) {
                if !must_be_before {
                    return false;
                }
            }
        }
    }
    true
}

fn middle_page_number(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn main() {
    let input = include_str!("input.txt");
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    // Parse rules
    let mut rules = HashMap::new();
    for line in rules_str.lines() {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        rules.insert((a, b), true);
    }

    // Parse updates
    let updates: Vec<Vec<usize>> = updates_str.lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    let mut sum_of_middle_page_numbers = 0;
    for update in updates {
        if is_valid_order(&update, &rules) {
            sum_of_middle_page_numbers += middle_page_number(&update);
        }
    }

    println!("Sum of middle page numbers: {}", sum_of_middle_page_numbers);
}