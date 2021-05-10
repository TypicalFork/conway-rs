#[derive(Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub grid: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl State {
    pub fn from(grid_2d: Vec<Vec<bool>>) -> Self {
        let width = grid_2d[0].len();
        let height = grid_2d.len();
        let mut grid = Vec::new();
        for mut row in grid_2d {
            grid.append(&mut row);
        }

        State {
            grid,
            width,
            height,
        }
    }

    pub fn random(chance: f64, dims: (usize, usize)) -> Self {
        let mut grid = vec![];
        for _ in 0..dims.1 {
            let mut row = vec![];
            for _ in 0..dims.0 {
                let random = rand::random::<f64>();
                if chance > random {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            grid.push(row);
        }
        State::from(grid)
    }

    fn check_neighbours(&self, cell: (usize, usize)) -> u8 {
        const ADJACENT: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut num_neighbours = 0;

        for adj_cell in &ADJACENT {
            // Convert cell items to i64 so that they doesn't overflow if the result is negative.
            let neighbour = (
                cell.0 as i64 + adj_cell.0 as i64,
                cell.1 as i64 + adj_cell.1 as i64,
            );
            if !(neighbour.1 >= self.width as i64
                || neighbour.1 < 0
                || neighbour.0 >= self.height as i64
                || neighbour.0 < 0)
                && self[neighbour.0 as usize][neighbour.1 as usize]
            {
                num_neighbours += 1;
            }
        }

        num_neighbours
    }

    pub fn next(&self) -> State {
        let mut grid = vec![false; self.width * self.height];
        let height = self.height;
        let width = self.width;

        for r in 0..height {
            for c in 0..width {
                let num_neighbours = self.check_neighbours((r, c));
                if (!self[r][c] && num_neighbours == 3)
                    || (self[r][c] && ((num_neighbours == 2) | (num_neighbours == 3)))
                {
                    grid[r * width + c] = true;
                }
            }
        }
        State {
            grid,
            width: self.width,
            height: self.height,
        }
    }
}

impl std::ops::Index<usize> for State {
    // Returns the row which allows for code like `State[2][0]`.
    type Output = [bool];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[self.width * index..self.width * (index + 1)]
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ALIVE: &str = "\u{2588}\u{2588}";
        const DEAD: &str = "  ";
        for r in 0..self.height {
            for c in 0..self.width {
                if self[r][c] {
                    write!(f, "{}", ALIVE)?;
                } else {
                    write!(f, "{}", DEAD)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        states: Vec<State>,
    }

    impl Setup {
        fn new() -> Self {
            let grids = vec![
                vec![
                    vec![false, false, true, true],
                    vec![true, true, false, false],
                    vec![true, true, false, false],
                    vec![false, true, false, true],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![false, true, false, false],
                ],
            ];

            let mut states = vec![];
            for grid in grids {
                states.push(State::from(grid));
            }

            Self { states }
        }
    }

    #[test]
    fn state_check_neighbours_test() {
        let setup = Setup::new();
        let full_tests = vec![
            vec![
                ((0, 2), 2),
                ((0, 3), 1),
                ((1, 0), 3),
                ((1, 1), 4),
                ((2, 0), 4),
                ((2, 1), 4),
                ((3, 1), 2),
                ((3, 3), 0),
            ],
            vec![((0, 2), 1), ((1, 2), 2), ((2, 1), 1)],
        ];

        let iter = setup.states.into_iter().zip(full_tests.into_iter());
        for (state, tests) in iter {
            for test in tests {
                let (cell, result) = test;
                assert_eq!(state.check_neighbours(cell), result);
            }
        }
    }

    #[test]
    fn state_next_step_test() {
        let setup = Setup::new();
        let result_grids: Vec<Vec<Vec<bool>>> = vec![
            vec![
                vec![false, true, true, false],
                vec![true, false, false, false],
                vec![false, false, false, false],
                vec![true, true, true, false],
            ],
            vec![
                vec![false, false, false, false],
                vec![false, true, true, false],
                vec![false, false, false, false],
            ],
        ];
        let mut full_results = Vec::new();
        for grid in result_grids {
            full_results.push(State::from(grid));
        }

        let iter = setup.states.into_iter().zip(full_results.into_iter());
        for (state, result) in iter {
            assert_eq!(state.next(), result);
        }
    }

    #[test]
    fn state_index_test() {
        let setup = Setup::new();

        for state in setup.states {
            for r in 0..state.height {
                for c in 0..state.width {
                    assert_eq!(state[r][c], state.grid[r * state.width + c]);
                }
            }
        }
    }
}
