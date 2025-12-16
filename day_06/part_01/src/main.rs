mod cephalopod_homework_maker;
use cephalopod_homework_maker::parse_values;
fn main() {
    let input = include_str!("./input.txt");
    println!("{}", parse_values(input));
}
