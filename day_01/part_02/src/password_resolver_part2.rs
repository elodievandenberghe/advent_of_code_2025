pub fn password_resolver_part2(input: &[&str]) -> u64 {
    let mut current_value: i64 = 50;
    let mut amount_0_reached = 0;
    for value in input {
        let direction = value.chars().next().unwrap();
        let amount: i64 = value[1..].parse().expect("f");

        match direction {
            'L' => {
                if current_value == 0 {
                    amount_0_reached += amount / 100;
                    current_value = 100 - (amount % 100);
                } else if (current_value - amount) < 1 {
                    amount_0_reached += if (amount - current_value) / 100 == 0 {
                        1
                    } else {
                        (amount - current_value) / 100
                    };
                    current_value = 100 - (amount % 100);
                } else {
                    current_value -= amount;
                }
            }
            'R' => {
                if current_value == 0 {
                    amount_0_reached += amount / 100;
                    current_value = amount % 100;
                } else if (current_value + amount) > 99 {
                    let remaining_distance = 100 - current_value;
                    amount_0_reached += if (amount + remaining_distance) / 100 == 0 {
                        1
                    } else {
                        (amount + remaining_distance) / 100
                    };
                    current_value = (amount - remaining_distance) % 100;
                } else {
                    current_value += amount;
                }
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
    fn test_r_simple_addition() {
        // current_value starts at 50, adding less than 50 to stay under 100
        let moves = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let result = password_resolver_part2(&moves);
        // 50 + 40 = 90 → amount_0_reached = 0
        assert_eq!(result, 0);
    }
}
