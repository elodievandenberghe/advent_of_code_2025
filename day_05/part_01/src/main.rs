mod fresh_ingredient_checker;
use fresh_ingredient_checker::check_ingredients;
fn main() {
    let input = include_str!("./input.txt");
    print!("{}", check_ingredients(input));
}
