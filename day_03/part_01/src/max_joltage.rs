pub fn process_joltage(input: &str) -> u64 {
    let chars_vec: Vec<char> = input.chars().collect();
    let mut max_value_index = 0;
    let mut max_value_index_2 = 0;
    for n in 1..chars_vec.len() - 1 {
        if chars_vec[max_value_index].to_digit(10) < chars_vec[n].to_digit(10) {
            max_value_index = n;
        }
    }
    let split_vec: &Vec<char> = &chars_vec[max_value_index + 1..].to_vec();
    for n in 1..split_vec.len() {
        if split_vec[max_value_index_2].to_digit(10) < split_vec[n].to_digit(10) {
            max_value_index_2 = n;
        }
    }
    let result_str = format!(
        "{}{}",
        chars_vec[max_value_index], split_vec[max_value_index_2]
    );
    let result: u64 = result_str.parse().expect("oops");
    return result;
}

pub fn start(input: &str) -> u64 {
    let vec: Vec<&str> = input.lines().collect();
    let mut endcount = 0;
    for item in vec {
        endcount += process_joltage(item)
    }
    endcount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";

        let result = start(input);
        println!("Result from start: {}", result);

        // Example assertion; replace with expected value if known
        assert_eq!(result, 357);
    }
}
