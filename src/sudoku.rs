use std::collections::BTreeSet as Set;

#[derive(Clone)]
pub struct SudokuPuzzle {
    // 0 is used to indicate the missing values
    data: [u8; 81],
}

impl SudokuPuzzle {
    pub fn new(rows: Vec<&str>) -> Self {
        SudokuPuzzle {
            data: {
                let mut data = [0; 81];
                for (row_idx, &row) in rows.iter().enumerate() {
                    for (column_idx, c) in row.chars().enumerate() {
                        data[row_idx * 9 + column_idx] = if c == ' ' {
                            0u8
                        } else {
                            char::to_digit(c, 10).unwrap() as u8
                        };
                    }
                }
                data
            },
        }
    }

    #[allow(unused)]
    pub fn example() -> Self {
        SudokuPuzzle::new(vec![
            "53  7    ",
            "6  195   ",
            " 98    6 ",
            "8   6   3",
            "4  8 3  1",
            "7   2   6",
            " 6    28 ",
            "   419  5",
            "    8  79",
        ])
    }

    #[allow(unused)]
    pub fn hard_example() -> Self {
        SudokuPuzzle::new(vec![
            "6    894 ",
            "9    61  ",
            " 7  4    ",
            "2  61    ",
            "      2  ",
            " 89  2   ",
            "    6   5",
            "       3 ",
            "8    16  ",
        ])
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        let v = self.data[row * 9 + col];
        if v == 0 {
            None
        } else {
            Some(v)
        }
    }

    pub fn set(&mut self, row: usize, col: usize, v: u8) {
        self.data[row * 9 + col] = v;
    }

    // the values that appear in the specified row
    fn row(&self, r: usize) -> Set<u8> {
        let mut s = Set::new();
        for &v in self.data[r * 9..=r * 9 + 8].iter() {
            s.insert(v);
        }
        s
    }

    // the values that appear in the specified column
    fn col(&self, c: usize) -> Set<u8> {
        let mut s = Set::new();
        for r in 0..=8 {
            s.insert(self.data[r * 9 + c]);
        }
        s
    }

    // the values that appear in the specified block
    fn blk(&self, r: usize, c: usize) -> Set<u8> {
        let mut s = Set::new();
        for x in 0..=2 {
            for y in 0..=2 {
                s.insert(self.data[(r * 3 + x) * 9 + (c * 3 + y)]);
            }
        }
        s
    }

    // all values that meet the criteria for a spot in the puzzle
    pub fn permitted_values(&self, r: usize, c: usize) -> Set<u8> {
        let row = self.row(r);
        let col = self.col(c);
        let blk = self.blk(r / 3, c / 3);
        let mut s = Set::new();
        for v in 1u8..=9 {
            if !row.contains(&v) && !col.contains(&v) && !blk.contains(&v) {
                s.insert(v);
            }
        }
        s
    }

    // spots in the puzzle that have not been assigned a value yet
    pub fn holes(&self) -> Vec<(usize, usize)> {
        (0..=8)
            .flat_map(|r| (0..=8).map(move |c| (r,c)))
            .filter(|(row, col)| self.get(*row, *col).is_none())
            .collect()
    }
}

impl std::fmt::Display for SudokuPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = {
            let mut d = ['X'; 81];
            for (i, n) in self.data.iter().enumerate() {
                d[i] = if *n == 0u8 {
                    ' '
                } else {
                    std::char::from_digit(*n as u32, 10).unwrap()
                }
            }
            d
        };
        writeln!(f, "-------------------")?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[9], d[10], d[11], d[12], d[13], d[14], d[15], d[16], d[17]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[18], d[19], d[20], d[21], d[22], d[23], d[24], d[25], d[26]
        )?;
        writeln!(f, "-------------------")?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[27], d[28], d[29], d[30], d[31], d[32], d[33], d[34], d[35]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[36], d[37], d[38], d[39], d[40], d[41], d[42], d[43], d[44]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[45], d[46], d[47], d[48], d[49], d[50], d[51], d[52], d[53]
        )?;
        writeln!(f, "-------------------")?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[54], d[55], d[56], d[57], d[58], d[59], d[60], d[61], d[62]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[63], d[64], d[65], d[66], d[67], d[68], d[69], d[70], d[71]
        )?;
        writeln!(
            f,
            "|{} {} {}|{} {} {}|{} {} {}|",
            d[72], d[73], d[74], d[75], d[76], d[77], d[78], d[79], d[80]
        )?;
        writeln!(f, "-------------------")
    }
}

pub struct Solver {
    puzzle: SudokuPuzzle,
}

impl Solver {
    pub fn new(puzzle: SudokuPuzzle) -> Self {
        Solver { puzzle }
    }

    pub fn solve(&self) -> Option<SudokuPuzzle> {
        // get all the places for which we need to select values
        let holes = self.puzzle.holes();

        // if there are no holes, then the puzzle is solved
        if holes.len() == 0 {
            return Some(self.puzzle.clone());
        }

        // for each "hole", get the values that are permissible and the number of permissible values
        let constraints = holes.iter().map(|(r,c)| {
            let constraint = self.puzzle.permitted_values(*r, *c);
            let size = constraint.len();
            ((*r, *c), constraint, size)
        }).collect::<Vec<((usize, usize), Set<u8>, usize)>>();

        // if any of the holes has only a single permissible value, then that is a forced constraint
        // in other words, we MUST choose that value

        let mut forced = Vec::new();
        let mut unforced = Vec::new();

        for ((r,c), constraint, size) in constraints {
            if size == 0 {
                // if any of the sets of permissible values are empty, then the puzzle is not
                // solvable
                return None;
            }
            if size == 1 {
                forced.push(((r,c), constraint, size));
            } else {
                unforced.push(((r,c),constraint, size))
            }
        }

        // now we'll create a copy of the current puzzle state and try filling in some of the
        // entries

        // when there are any forced constraints, we can fill in those values and try to solve the
        // resulting puzzle
        if forced.len() > 0 {
            let mut p = self.puzzle.clone();

            for ((r, c), constraint , _) in forced {
                for v in constraint {
                    p.set(r, c, v);
                }
            }

            return Solver::new(p).solve();
        }

        // otherwise, we'll have to try choosing values for one of the holes until we find a
        // solution
        else {
            //
            unforced.sort_by_key(|(_,_,size)| *size);

            // we know there's at least one item here, otherwise we'd be done
            let ((r, c), constraint, _) = unforced.first().unwrap();

            for v in constraint {
                let mut p = self.puzzle.clone();
                p.set(*r, *c, *v);
                if let solution@Some(_) = Solver::new(p).solve() {
                    return solution
                }
            }

            None
        }
    }

    pub fn run(&self) {
        if let Some(p) = self.solve() {
            println!("{}", p);
        } else {
            println!("no solution");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet as Set;
    use std::iter::FromIterator;
    use crate::sudoku::SudokuPuzzle;

    #[test]
    fn get_01() {
        let p = SudokuPuzzle::example();
        assert_eq!(p.get(0,0), Some(5));
    }

    #[test]
    fn get_02() {
        let p = SudokuPuzzle::example();
        assert_eq!(p.get(5,4), Some(2));
    }

    #[test]
    fn get_03() {
        let p = SudokuPuzzle::example();
        assert_eq!(p.get(2, 6), None);
    }

    #[test]
    fn set_01() {
        let mut p = SudokuPuzzle::example();
        p.set(5, 4, 8);
        assert_eq!(p.get(5, 4), Some(8));
    }

    #[test]
    fn set_02() {
        let mut p = SudokuPuzzle::example();
        p.set(2, 6, 3);
        assert_eq!(p.get(2, 6), Some(3));
    }

    #[test]
    fn row_01() {
        let p = SudokuPuzzle::example();
        let r = p.row(3);
        assert_eq!(r, Set::from_iter(vec![8, 0, 0, 0, 6, 0, 0, 0, 3].iter().map(|x|*x)));
    }

    #[test]
    fn row_02() {
        let p = SudokuPuzzle::example();
        let r = p.row(8);
        assert_eq!(r, Set::from_iter(vec![0, 0, 0, 0, 8, 0, 0, 7, 9].iter().map(|x| *x)));
    }

    #[test]
    fn col_01() {
        let p = SudokuPuzzle::example();
        let c = p.col(3);
        assert_eq!(c, Set::from_iter(vec![0, 1, 0, 0, 8, 0, 0, 4, 0].iter().map(|x|*x)));
    }

    #[test]
    fn col_02() {
        let p = SudokuPuzzle::example();
        let c = p.col(0);
        assert_eq!(c, Set::from_iter(vec![5, 6, 0, 8, 4, 7, 0, 0, 0].iter().map(|x|*x)));
    }

    #[test]
    fn blk_01() {
        let p = SudokuPuzzle::example();
        let b = p.blk(0, 0);
        assert_eq!(b, Set::from_iter(vec![5, 3, 0, 6, 0, 0, 0, 9, 8].iter().map(|x|*x)));
    }

    #[test]
    fn blk_02() {
        let p = SudokuPuzzle::example();
        let b = p.blk(2, 1);
        assert_eq!(b, Set::from_iter(vec![0, 0, 0, 4, 1, 9, 0, 8, 0].iter().map(|x|*x)));
    }

    #[test]
    fn permitted_values_01() {
        let p = SudokuPuzzle::example();
        assert_eq!(
            p.permitted_values(4, 4),
            Set::from_iter(vec![5].iter().map(|x|*x))
        );
    }

    #[test]
    fn permitted_values_02() {
        let p = SudokuPuzzle::example();
        assert_eq!(
            p.permitted_values(8, 0),
            Set::from_iter(vec![1,2,3].iter().map(|x|*x))
        );
    }

    #[test]
    fn holes_01() {
        let p = SudokuPuzzle::example();
        let actual_holes = vec![
            vec![2, 3, 5, 6, 7, 8],
            vec![1, 2, 6, 7, 8],
            vec![0, 3, 4, 5, 6, 8],

            vec![1, 2, 3, 5, 6, 7],
            vec![1, 2, 4, 6, 7],
            vec![1, 2, 3, 5, 6, 7],

            vec![0, 2, 3, 4, 5, 8],
            vec![0, 1, 2, 6, 7],
            vec![0, 1, 2, 3, 5, 6],
        ];
        let holes = p.holes();
        for (row, data) in actual_holes.iter().enumerate() {
            for &col in data {
                assert!(holes.contains(&(row, col)));
            }
        }
    }
}