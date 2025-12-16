pub fn make_homework(input: Vec<Vec<&str>>) -> u64 {
    let mut end_result = 0;
    for j in 0..input[0].len() {
        let mut temp_arr: Vec<String> = vec![];
        for n in 0..input.len() {
            temp_arr.push(input[n][j].to_string());
        }
        let u64_array: Vec<u64> = temp_arr[..temp_arr.len() - 1]
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let operand = &temp_arr[temp_arr.len() - 1];
        match operand.as_str() {
            "+" => end_result += u64_array.iter().sum::<u64>(),
            "*" => end_result += u64_array.iter().product::<u64>(),
            _ => {}
        }
    }

    end_result
}

pub fn parse_values(input: &str) -> u64 {
    let array: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    make_homework(array)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_homework() {
        let input = "\
123 328 51 64
45 64 387 23
6 98 215 314
* + * +";

        let result = parse_values(input);
        assert_eq!(result, 4277556);
    }
}
