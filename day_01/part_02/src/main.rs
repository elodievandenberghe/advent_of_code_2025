mod password_resolver_part2;
use password_resolver_part2::password_resolver_part2;
fn main() {
    let input = include_str!("./input_part2.txt");
    let lines: Vec<&str> = input.lines().collect();

    print!("{}", password_resolver_part2(&lines));
}
