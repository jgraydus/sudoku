mod sudoku;

use sudoku::*;

fn main() {
    Solver::new(SudokuPuzzle::example()).run();

    Solver::new(SudokuPuzzle::hard_example()).run();
}
