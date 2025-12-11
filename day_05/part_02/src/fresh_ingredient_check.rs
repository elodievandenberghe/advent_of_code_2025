pub fn check_ingredients(input: &str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let top: Vec<&str> = parts[0].lines().collect();
    let mut count = 0;
    let mut arr: Vec<(u64, u64)> = vec![];

    for n in 0..top.len() {
        let start: u64 = top[n][..top[n].find("-").unwrap()].parse().unwrap();
        let end: u64 = top[n][top[n].find("-").unwrap() + 1..].parse().unwrap();
        let tuple = (start, end);
        arr.push(tuple);
    }

    arr.sort_by_key(|&(start, _end)| start);

    let mut n = 0;
    while n < arr.len() {
        let mut j = n + 1;
        while j < arr.len() {
            let (start, end) = arr[n];
            let (start2, end2) = arr[j];
            if end >= start2 && end2 >= start {
                arr[n] = (start.min(start2), end.max(end2));
                arr.remove(j);
            } else {
                j += 1;
            }
        }
        n += 1;
    }
    for value in arr {
        count += (value.1 - value.0) + 1;
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
        assert_eq!(result, 14);
    }
}
