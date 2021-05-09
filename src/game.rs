#[derive(Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub grid: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl std::ops::Index<usize> for State {
    type Output = [bool];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[self.width * index..self.width * (index + 1)]
    }
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
            if !(neighbour.1 >= self.height as i64
                || neighbour.1 < 0
                || neighbour.0 >= self.width as i64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_neighbours_test() {
        let grid = vec![
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, true, false, true],
        ];
        let state = State::from(grid);
        let tests = vec![
            ((0, 2), 2),
            ((0, 3), 1),
            ((1, 0), 3),
            ((1, 1), 4),
            ((2, 0), 4),
            ((2, 1), 4),
            ((3, 1), 2),
            ((3, 3), 0),
        ];

        for test in tests {
            let (cell, result) = test;
            assert_eq!(state.check_neighbours(cell), result);
        }
    }

    #[test]
    fn next_step_test() {
        let grid = vec![
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, true, false, true],
        ];
        let state = State::from(grid);
        let result_grid: Vec<Vec<bool>> = vec![
            vec![false, true, true, false],
            vec![true, false, false, false],
            vec![false, false, false, false],
            vec![true, true, true, false],
        ];
        let result = State::from(result_grid);

        assert_eq!(state.next(), result);
    }

    #[test]
    fn state_index_test() {
        let state = State {
            grid: vec![true, false, true],
            width: 1,
            height: 3,
        };

        assert_eq!(state[1][0], false);
    }
}
