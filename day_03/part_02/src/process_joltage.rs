pub fn process_joltage(input: &str) -> u64 {
    let chars_vec: Vec<char> = input.chars().collect();
    let mut result: Vec<char> = vec![];
    let mut begin_search_index = 0;
    let mut max_search_length = chars_vec.len() - 12;
    let mut max_value_index = 0;

    while result.len() != 12 {
        max_value_index = begin_search_index;
        for n in begin_search_index..=max_search_length {
            if chars_vec[max_value_index].to_digit(10) < chars_vec[n].to_digit(10) {
                max_value_index = n;
            }
        }
        result.push(chars_vec[max_value_index]);
        begin_search_index = max_value_index + 1;
        max_search_length += 1;
    }
    result.iter().collect::<String>().parse::<u64>().unwrap()
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
    fn test_process_joltage() {
        assert_eq!(process_joltage("987654321111111"), 987654321111);
        assert_eq!(process_joltage("811111111111119"), 811111111119);
        assert_eq!(process_joltage("234234234234278"), 434234234278);
        assert_eq!(process_joltage("818181911112111"), 888911112111);
    }
    #[test]
    fn test_start() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(start(input), 3121910778619);
    }
}
