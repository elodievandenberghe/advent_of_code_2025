pub fn check_paper(input: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let rows = input.len();
    if rows == 0 {
        return 0;
    }
    let cols = input[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == '@' {
                let mut count = 0;

                // Check each neighbor carefully
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
                    result += 1;
                }
            }
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
