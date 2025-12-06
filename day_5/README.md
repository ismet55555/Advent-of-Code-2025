# Scribbles and Notes for This Day

## Part 1

- Input
  - Fresh ingredient ID ranges (inclusive)
    - Ranges can overlap
    - Fresh if it is in any range
  - Blank line separator
  - Available ingredient IDs

### Plan of Attack 1

- Create a range
- Add to a single vector: `Vec<i64>`
- Remove duplicates
- Repeat until done
- Check if ingredient number is included in the vector
- **NO! Will need WAY too much memory!**

- OR

- Loop through all ranges
- For each range, check if the ingredient is within the boundaries of the range

## Part 2

- Ranges from input only
- Get the full possible fresh ingredient number from ranges
- They can overlap

### Plan of Attack 2

- Find how many IDs are in range (max-min)
- Finding overlap is tricky part
  - dot product??
  - Can I even hold this in memory??

- Compare two ranges
- Check if minimum of second what falls in range of first
- If so, combine to *make a new range*
  - minimum of first to maximum of second
- Iterate through all

```txt
RANGES:
          -------        ----------
-----  ------  ------      -----


COMBINED:
-----  --------------    ----------
```
