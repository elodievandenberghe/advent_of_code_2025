mod range_checker;

use range_checker::start_meow;

fn main() {
    let input = include_str!("./input_part_01.txt");
    start_meow(input);
    print!("{}", start_meow(input));
}
