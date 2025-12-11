pub fn check_ingredients(input: &str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let top: Vec<&str> = parts[0].lines().collect();
    let bottom: Vec<u64> = parts[1]
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();
    let mut count = 0;

    for val in bottom {
        let mut is_not_fresh = true;
        let mut n = 0;
        while is_not_fresh && n < top.len() {
            let range = &top[n];

            let start = range[..range.find("-").unwrap()].parse().unwrap();
            let end = range[range.find("-").unwrap() + 1..].parse().unwrap();
            if val >= start && val <= end {
                is_not_fresh = false;
                count += 1;
            }
            n += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_paper() {
        let raw = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = check_ingredients(raw);
        assert_eq!(result, 3);
    }
}
