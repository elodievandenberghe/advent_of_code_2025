mod laboratory;
use laboratory::parse_values;
fn main() {
    let input = include_str!("./input.txt");
    println!("{}", parse_values(input));
}
