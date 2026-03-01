pub fn set_splits(mut input: Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for n in 1..input.len() {
        for j in 0..input[n].len() {
            if input[n][j] == '^' && input[n - 1][j] == '|' {
                input[n][j - 1] = '|';
                input[n][j + 1] = '|';
                count += 1;
            } else if (input[n - 1][j] == '|' || input[n - 1][j] == 'S') && input[n][j] != '^' {
                input[n][j] = '|';
            }
        }
    }
    count
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
        assert_eq!(result, 21);
    }
}
