# sudoku solver

This program solves sudoku puzzles very quickly by using constraints. It's a recursive algorithm that works like this:
- base case 1: there are no empty cells left. the puzzle is solved
- base case 2: there are empty cells, but one or more of them have no valid choice of value. the puzzle has no solution.
- recursive case 1: there are one or more empty cells in which only a single value will work. those cells are assigned 
  the required values and the resulting puzzle is recursively solved
- recursive case 2: every empty cell could have multiple values. one of the most constrained (i.e. the least number of possible 
  values) cells is chosen. for each possible value, recursively attempt to solve the puzzle with the cell assigned that value.
  if none of the values work, then the puzzle has no solution

![sudoku puzzle](https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg/250px-Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg.png)

```
-------------------
|5 3 4|6 7 8|9 1 2|
|6 7 2|1 9 5|3 4 8|
|1 9 8|3 4 2|5 6 7|
-------------------
|8 5 9|7 6 1|4 2 3|
|4 2 6|8 5 3|7 9 1|
|7 1 3|9 2 4|8 5 6|
-------------------
|9 6 1|5 3 7|2 8 4|
|2 8 7|4 1 9|6 3 5|
|3 4 5|2 8 6|1 7 9|
-------------------
```
