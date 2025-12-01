pub fn password_resolver_part2(input: &[&str]) -> u64 {
    let mut current_value: i64 = 50;
    let mut amount_0_reached = 0;
    for value in input {
        let direction = value.chars().next().unwrap();
        let amount: i64 = value[1..].parse().expect("bleeehh :pppp");
        match direction {
            'L' => {
                if current_value != 0 {
                    let value_to_add = (amount - current_value) as f64 / 100.0;
                    amount_0_reached += value_to_add.ceil() as i64;
                }
                current_value = (current_value - amount).rem_euclid(100);
            }
            'R' => {
                if current_value != 0 {
                    let distance = 100 - current_value;
                    let value_to_add = (amount - distance) as f64 / 100.0;
                    amount_0_reached += value_to_add.ceil() as i64;
                }
                current_value = (current_value + amount).rem_euclid(100);
            }
            _ => {}
        }
    }
    amount_0_reached as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_test_values() {
        let moves = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        assert_eq!(password_resolver_part2(&moves), 6);
    }
}
