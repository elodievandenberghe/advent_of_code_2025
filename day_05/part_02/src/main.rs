mod fresh_ingredient_check;
use fresh_ingredient_check::check_ingredients;
fn main() {
    let input = include_str!("./input.txt");
    print!("{}", check_ingredients(input));
}
