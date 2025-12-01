# Scribbles and Notes for This Day

https://adventofcode.com/2025/day/1

## Part 1

- A cycling lock
- Numbers/clicks 0-99
- Rotates around past 0
- Instructions LX, RX, where X is number of clicks on cycling lock
  - LX - Move left X clicks
  - RX - Move right X clicks
- For every instruction, check if it lands on zero
- Final answer is number of zeros

### Plan of Attack

- Load input
- Seperate input in direction and number of clicks
- Keep a constant vector from 0-99, that cycles
- Cycle through istructions
- For each instruction rotate the vector
- Check if vector is at zero - If at zero add to a counter variable

## Part 2

...
