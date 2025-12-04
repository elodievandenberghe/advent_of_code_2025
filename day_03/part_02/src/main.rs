mod process_joltage;
use process_joltage::start;
fn main() {
    let input = include_str!("./input.txt");
    println!("{}", start(input));
}
