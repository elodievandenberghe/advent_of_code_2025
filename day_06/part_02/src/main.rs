mod cephalopod_part2;
use cephalopod_part2::parse_values;
fn main() {
    let input = include_str!("./input.txt");
    println!("{}", parse_values(input));
}
