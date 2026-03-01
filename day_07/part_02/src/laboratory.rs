use core::str;

pub fn set_splits(mut grid: Vec<Vec<char>>) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut paths = vec![vec![0u64; cols]; rows];

    for j in 0..cols {
        if grid[0][j] == 'S' {
            paths[0][j] = 1;
        }
    }
    for r in 0..rows - 1 {
        for c in 0..cols {
            let current_paths = paths[r][c];
            if current_paths == 0 {
                continue;
            }

            if grid[r + 1][c] == '^' {
                if c > 0 {
                    paths[r + 1][c - 1] += current_paths;
                }
                if c < cols - 1 {
                    paths[r + 1][c + 1] += current_paths;
                }
            } else {
                paths[r + 1][c] += current_paths;
            }
        }
    }

    paths[rows - 1].iter().sum()
}

pub fn parse_values(input: &str) -> u64 {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    set_splits(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_homework() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let result = parse_values(input);
        assert_eq!(result, 40);
    }
}
