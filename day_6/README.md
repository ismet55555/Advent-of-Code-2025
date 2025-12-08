# Scribbles and Notes for This Day

## Part 1

- Parsing exercise
- Parse all vertically based on whitespace
  - Number 1
  - Number 2
  - Number 3
  - Operation
- `Number 1 <op> Number 2 <op> Number 3`

## Part 2

- Now the white space matters.
- Parsing will be a little more tricky
- Parsing
  - Split based on one single space (` `) - NO
  - Operation symbol is always on the left side of the numbers lined up
  - Parse based on operation symbols
  - Use string slices based on operation symbol index

- Loop operation vector
- Find index of "*"
- Same index, loop up
- If any of the nums index have number, add
- Repeat until no no nums found, all space
-
- At end trim and parse to i32
