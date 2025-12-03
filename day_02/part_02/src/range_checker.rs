pub fn is_sequence_invalid(sequence: &str) -> bool {
    let sequence_length = sequence.len();

    for n in 1..=sequence_length / 2 {
        let a = &sequence[..n];
        let pattern = a.repeat(sequence_length / n);
        if pattern == sequence {
            return true;
        }
    }
    false
}

pub fn iterate_through_range(range: &str) -> u64 {
    let (first, last) = range
        .trim()
        .split_once('-')
        .expect("range should contain '-'");
    let start = first.parse::<u64>().expect("invalid number");
    let end = last.parse::<u64>().expect("invalid number");

    (start..=end)
        .filter(|&n| is_sequence_invalid(&n.to_string()))
        .sum()
}
pub fn start_meow(input: &str) -> u64 {
    input
        .split(',')
        .map(|part| iterate_through_range(part))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterate_through_range() {
        // assert!(!is_sequence_invalid("abc"));
        assert_eq!(
            start_meow(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
