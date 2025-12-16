pub fn parse_values(input: &str) -> u64 {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let values = &array[..array.len() - 1];
    let operands = array.last().unwrap();

    let mut end_result: u64 = 0;
    let mut numbers_arr: Vec<u64> = vec![];

    for j in (0..values[0].len()).rev() {
        let column: String = values
            .iter()
            .map(|v| v[j])
            .filter(|c| !c.is_whitespace())
            .collect();

        if !column.is_empty() {
            let num = column.parse::<u64>().unwrap();
            numbers_arr.push(num);
        }

        if j < operands.len() && operands[j] != ' ' {
            match operands[j] {
                '+' => {
                    end_result += numbers_arr.iter().sum::<u64>();
                }
                '*' => {
                    end_result += numbers_arr.iter().product::<u64>();
                }
                _ => {}
            }
            numbers_arr.clear();
        }
    }

    if !numbers_arr.is_empty() {
        end_result += numbers_arr.iter().sum::<u64>();
    }

    end_result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_homework() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

        let result = parse_values(input);
        assert_eq!(result, 3263827);
    }
}
