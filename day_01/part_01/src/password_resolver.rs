pub fn password_resolver(input: Vec<&str>) -> u16 {
    let mut current_value: i64 = 50;
    let mut amount_0_reached = 0;

    for value in input {
        let direction = value.chars().next().unwrap();
        let amount: i64 = value[1..].parse().expect("not parsable");

        match direction {
            'L' => {
                current_value = (current_value - amount).rem_euclid(100);
            }
            'R' => {
                current_value = (current_value + amount).rem_euclid(100);
            }
            _ => {}
        }

        if current_value == 0 {
            amount_0_reached += 1;
        }
    }

    amount_0_reached
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_with_test_values() {
        let moves = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        assert_eq!(password_resolver(moves), 3);
    }
}
