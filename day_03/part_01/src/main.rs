mod max_joltage;
use max_joltage::start;
fn main() {
    let input = include_str!("./input.txt");
    println!("{}", start(input));
}
