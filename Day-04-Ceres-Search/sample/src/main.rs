use std::io::{self, BufRead};

const WORD: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (1, 1),   // Down-Right Diagonal
    (1, -1),  // Down-Left Diagonal
    (0, -1),  // Left
    (-1, 0),  // Up
    (-1, -1), // Up-Left Diagonal
    (-1, 1),  // Up-Right Diagonal
];

fn find_word_count(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &DIRECTIONS {
                if is_word_present(grid, &word_chars, row, col, dr, dc, word_len) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_word_present(
    grid: &[Vec<char>],
    word_chars: &[char],
    start_row: usize,
    start_col: usize,
    dr: isize,
    dc: isize,
    word_len: usize,
) -> bool {
    let mut r = start_row as isize;
    let mut c = start_col as isize;

    for &ch in word_chars {
        if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
            return false;
        }
        if grid[r as usize][c as usize] != ch {
            return false;
        }
        r += dr;
        c += dc;
    }

    true
}

fn main() {
    // Read the input grid from stdin
    let stdin = io::stdin();
    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let count = find_word_count(&grid, WORD);
    println!("{}", count);
}
