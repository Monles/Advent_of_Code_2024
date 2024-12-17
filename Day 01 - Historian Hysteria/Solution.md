# Overview
## 1. Input Reading:
- The program reads the `input.txt` file line by line.
- It parses two integers from each line and appends them to separate vectors, `left_numbers` and `right_numbers`.

## 2. Sorting:
- Both `left_numbers` and `right_numbers` are sorted independently using `.sort()`.

## 3. Pairing and Distance Calculation:
- The sorted lists are zipped together pairwise.
- For each pair, the absolute difference `abs(a - b)` is calculated and summed.

## 4. Output:
The total distance is printed to the terminal.



# Reviews
## Main Function
```rust
fn main() -> io::Result<()> {
```
The main function returns a Result, which allows us to gracefully handle errors during file operations.

