mod password_resolver;
use password_resolver::password_resolver;
fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();

    print!("{}", password_resolver(lines));
}
