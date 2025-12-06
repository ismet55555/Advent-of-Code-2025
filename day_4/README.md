# Scribbles and Notes for This Day

## Part 1

- `@` are rolls of paper
- Only get positions where `@` have fewer than four other `@` around them
- Count how many those are

- Load input as a Vector of vectors of characters
- Or a Grid struct given number of rows and columns or a given structure?
  - given input as grid data: `data: Vec<Vec<i32>>`
  - `get_neighbours() -> Vec<Char>`
  - `count_surrounding_rolls() -> i32`
  - Loop through each and count

## Part 2

- Keep repeating the remove of available rolls until nothing cna be removed
- That is, removed papers equal zero
