# Scribbles and Notes for This Day

<https://adventofcode.com/2025/day/3>

## Part 1

- Split input by new line
- Find largest number in row - First digit
- Find largest number in set right of first digit - Second digit
- If largest number is all the way to right, find second digit on left
  - Second digit becomes first digit
  - Initial first digit becomes second digit
- Combine digit to make an integer
- Add all integers for each row

- Finding max algorithm: Simple linear, one for loop
- Rust way. `vec.iter().max()`

## Part 2

- **(INCOMPLETE!)**

- Pick 12 digits that make the largest number when concatenated

1. First - Largest digit that _allows 11 more to the right_
2. Second - Largest digit to the right that _allows 10 more to the right_
3. Third - 12 - i - 1 available

**Key constraint to check at each step:**

> When you're at position i in your result (need 12 - i more digits),
> and considering a digit at position j in the original array, ask:
> "Are there at least 12 - i - 1 more digits available after position j?"

**Setup:**

- Original: `818181911112111` (15 digits total)
- Need to pick: 12 digits
- Building result left-to-right

---

**Position 0 in result (need 12 more digits total, so need 11 after this):**

Scan through the array:

- Index 0: `8` - are there 11+ digits after index 0? Yes (14 remaining). Candidate: `8`
- Index 1: `1` - smaller than `8`, skip
- Index 2: `8` - equal to current best, but already have `8` from index 0
- Index 3: `1` - smaller, skip
- Index 4: `8` - equal, skip
- Index 5: `1` - smaller, skip
- Index 6: `9` - **LARGER than `8`!** Are there 11+ digits after index 6? Let's count: positions 7-14 = 8 digits. **Not enough!** Can't use this.
- Continue scanning... no better options

**Pick:** `8` at index 0
**Result so far:** `8`
**Continue from:** index 1

---

**Position 1 in result (need 11 more, so need 10 after this):**

Scan from index 1 onward:

- Index 1: `1` - are there 10+ digits after? Yes (13 remaining). Candidate: `1`
- Index 2: `8` - larger! Are there 10+ after index 2? Positions 3-14 = 12 digits. Yes! Better candidate: `8`
- Index 3-5: smaller or equal, skip efficiently
- Index 6: `9` - larger! Are there 10+ after index 6? Positions 7-14 = 8 digits. **Not enough!**
- No better than `8` at index 2

**Pick:** `8` at index 2
**Result so far:** `88`
**Continue from:** index 3

---

**Position 2 in result (need 10 more, so need 9 after this):**

Scan from index 3 onward:

- Index 3: `1`
- Index 4: `8` - larger, are there 9+ after index 4? Positions 5-14 = 10 digits. Yes!
- Index 6: `9` - larger! Are there 9+ after index 6? Positions 7-14 = 8 digits. **Not enough!**

**Pick:** `8` at index 4
**Result so far:** `888`
**Continue from:** index 5

---

**Position 3 in result (need 9 more, so need 8 after this):**

Scan from index 5 onward:

- Index 5: `1`
- Index 6: `9` - larger! Are there 8+ after index 6? Positions 7-14 = 8 digits. **Exactly enough!**

**Pick:** `9` at index 6
**Result so far:** `8889`
**Continue from:** index 7

**Positions 4-11 (need remaining 8 digits):**

Now we've used index 6, and need 8 more digits. From index 7 onward,
there are exactly 8 digits left: `11112111`

Since we need exactly as many as are remaining, we must take all of them.

**Pick:** All remaining: `11112111`
**Final result:** `888911112111`

---

**Summary:**

- Picked indices: 0, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14
- Skipped indices: 1, 3, 5
- Result: `888911112111`

**The key insight:**
At each position, find the largest digit you can use
while ensuring enough digits remain to complete the 12-digit number!
