type State = Vec<Vec<bool>>;

fn check_neighbours(state: &State, cell: (usize, usize)) -> u8 {
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

    let height = state.len();
    let width = state[0].len();

    for adj_cell in &ADJACENT {
        // Convert cell items to i64 so that they doesn't overflow if the result is negative.
        let neighbour = (
            cell.0 as i64 + adj_cell.0 as i64,
            cell.1 as i64 + adj_cell.1 as i64,
        );
        if !(neighbour.1 >= height as i64
            || neighbour.1 < 0
            || neighbour.0 >= width as i64
            || neighbour.0 < 0)
            && state[neighbour.0 as usize][neighbour.1 as usize]
        {
            num_neighbours += 1;
        }
    }

    num_neighbours
}

pub fn next_step(state: State) -> State {
    let height = state.len();
    let width = state[0].len();

    let mut result = vec![vec![false; width]; height];

    for r in 0..height {
        for c in 0..width {
            let num_neighbours = check_neighbours(&state, (r, c));
            if (!state[r][c] && num_neighbours == 3)
                || (state[r][c] && ((num_neighbours == 2) | (num_neighbours == 3)))
            {
                result[r][c] = true;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_neighbours_test() {
        let state = vec![
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, true, false, true],
        ];
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
            assert_eq!(check_neighbours(&state, cell), result);
        }
    }

    #[test]
    fn next_step_test() {
        let state = vec![
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, true, false, true],
        ];
        let result: Vec<Vec<bool>> = vec![
            vec![false, true, true, false],
            vec![true, false, false, false],
            vec![false, false, false, false],
            vec![true, true, true, false],
        ];

        assert_eq!(next_step(state), result);
    }
}
