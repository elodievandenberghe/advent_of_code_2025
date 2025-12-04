mod check_2darray;
use check_2darray::start;
fn main() {
    let input = include_str!("./input.txt");
    print!("{}", start(input));
}
