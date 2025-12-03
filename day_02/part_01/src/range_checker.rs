pub fn is_sequence_invalid(input: u64) -> bool {
    let input_str = input.to_string();
    let string_length = input_str.len();

    if string_length % 2 != 0 {
        false
    } else {
        let string_first_half = &input_str[0..(string_length) / 2];
        let string_second_half = &input_str[(string_length / 2)..];
        if string_first_half == string_second_half {
            true
        } else {
            false
        }
    }
}

pub fn iterate_through_range(range: &str) -> u64 {
    let (first, last) = range.trim().split_once('-').unwrap();
    let start: u64 = first.parse().unwrap();
    let end: u64 = last.parse().unwrap();
    let mut invalid_ranges_count = 0;

    for sequence in start..=end {
        if is_sequence_invalid(sequence) {
            invalid_ranges_count += sequence;
        }
    }
    invalid_ranges_count
}

pub fn start_meow(input: &str) -> u64 {
    let mut count = 0;
    for part in input.split(',') {
        count += iterate_through_range(part);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_test_values() {
        assert_eq!(
            start_meow(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }
}
