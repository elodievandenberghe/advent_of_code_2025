pub fn check_paper(inputstr: Vec<Vec<char>>) -> u64 {
    let mut input = inputstr;
    let mut result = 0;
    let rows = input.len();
    if rows == 0 {
        return 0;
    }
    let cols = input[0].len();
    let mut no_moves = false;
    let mut previous_count;
    while !no_moves {
        previous_count = result;
        for i in 0..rows {
            for j in 0..cols {
                if input[i][j] == '@' {
                    let mut count = 0;

                    if i > 0 && input[i - 1][j] == '@' {
                        count += 1;
                    }
                    if i > 0 && j + 1 < cols && input[i - 1][j + 1] == '@' {
                        count += 1;
                    }
                    if j + 1 < cols && input[i][j + 1] == '@' {
                        count += 1;
                    }
                    if i + 1 < rows && j + 1 < cols && input[i + 1][j + 1] == '@' {
                        count += 1;
                    }
                    if i + 1 < rows && input[i + 1][j] == '@' {
                        count += 1;
                    }
                    if i + 1 < rows && j > 0 && input[i + 1][j - 1] == '@' {
                        count += 1;
                    }
                    if j > 0 && input[i][j - 1] == '@' {
                        count += 1;
                    }
                    if i > 0 && j > 0 && input[i - 1][j - 1] == '@' {
                        count += 1;
                    }

                    if count < 4 {
                        input[i][j] = '.';
                        result += 1;
                    }
                }
            }
        }
        if previous_count == result {
            no_moves = true;
        }
    }
    result
}

pub fn start(input: &str) -> u64 {
    let array = input.lines().map(|line| line.chars().collect()).collect();

    check_paper(array)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_paper() {
        let raw = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = start(raw);

        assert_eq!(result, 2);
    }
}
