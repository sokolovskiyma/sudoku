use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Sudoku(pub Vec<Vec<u8>>);

impl Sudoku {
    pub fn from_vec(vec: Vec<Vec<u8>>) -> Sudoku {
        Sudoku(vec)
    }

    pub fn solve(&self) -> Option<Sudoku> {
        let mut solution = self.clone();
        if solution.solve_helper() {
            return Some(solution);
        }
        None
    }

    fn solve_helper(&mut self) -> bool {
        let mut min_row: usize = 10;
        let mut min_column: usize = 10;
        let mut min_values: HashSet<u8> = HashSet::new();

        loop {
            min_row = 10;
            for r in 0..9 {
                for c in 0..9 {
                    if self.0[r][c] != 0 {
                        continue;
                    };

                    let possible_values = self.find_possible_values(r, c);
                    let possible_values_count = possible_values.len();

                    if possible_values_count == 0 {
                        return false;
                    };

                    if possible_values_count == 1 {
                        self.0[r][c] = possible_values.iter().next().unwrap().clone();
                    };

                    if min_row > 9 || possible_values_count < min_values.len() {
                        min_row = r;
                        min_column = c;
                        min_values = possible_values;
                    };
                }
            }

            if min_row == 10 {
                return true;
            } else if min_values.len() > 1 {
                break;
            };
        }
        for v in min_values {
            let mut solution_copy = self.clone();
            solution_copy.0[min_row][min_column] = v;
            if solution_copy.solve_helper() {
                self.0 = solution_copy.0.clone();
                return true;
            }
        }

        false
    }

    fn find_possible_values(&self, row: usize, column: usize) -> HashSet<u8> {
        let mut values: HashSet<u8> = (1..10).collect();

        values = values
            .difference(&self.get_row_values(row))
            .copied()
            .collect();
        values = values
            .difference(&self.get_column_values(column))
            .copied()
            .collect();
        values = values
            .difference(&self.get_block_values(row, column))
            .copied()
            .collect();

        values
    }

    fn get_row_values(&self, row: usize) -> HashSet<u8> {
        self.0[row].clone().into_iter().collect()
    }

    fn get_column_values(&self, column: usize) -> HashSet<u8> {
        let mut values: HashSet<u8> = HashSet::new();
        for r in 0..9 {
            values.insert(self.0[r][column]);
        }
        values
    }

    fn get_block_values(&self, row: usize, column: usize) -> HashSet<u8> {
        let mut values: HashSet<u8> = HashSet::new();
        let block_row_start = 3 * (row / 3);
        let block_column_start = 3 * (column / 3);

        for r in 0..3 {
            for c in 0..3 {
                values.insert(self.0[block_row_start + r][block_column_start + c]);
            }
        }

        values
    }
}

impl Display for Sudoku {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut formated = "┌─────────┬─────────┬─────────┐".to_owned();
        if self.0.len() == 9 {
            for row in self.0.clone().iter().enumerate() {
                formated.push_str("\n");
                if row.0 != 0 && row.0 % 3 == 0 {
                    formated.push_str("├─────────┼─────────┼─────────┤\n");
                }
                if row.1.len() == 9 {
                    for index in row.1.iter().enumerate() {
                        if index.0 % 3 == 0 {
                            formated.push_str("│");
                        }
                        formated.push_str(" ");
                        formated.push_str(&index.1.to_string());
                        formated.push_str(" ");
                        if index.0  == 8 {
                            formated.push_str("│");
                        }
                    }
                } else {
                    formated = "Нет решения".to_string();
                    break;
                }
            }
            formated.push_str("\n└─────────┴─────────┴─────────┘");
        } else {
            formated = "Нет решения".to_string();
        }
        write!(f, "{}", formated)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve() {
        let sudoku: Sudoku = Sudoku::from_vec(vec![
            vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 3, 6, 0, 0, 0, 0, 0],
            vec![0, 7, 0, 0, 9, 0, 2, 0, 0],
            vec![0, 5, 0, 0, 0, 7, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 5, 7, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 3, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 6, 8],
            vec![0, 0, 8, 5, 0, 0, 0, 1, 0],
            vec![0, 9, 0, 0, 0, 0, 4, 0, 0],
        ]);

        // assert_eq!(sudoku.solve(), Sudoku::from_vec())
    }
}
