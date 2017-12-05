extern crate adventofcode;

use adventofcode::day01;

fn main() {
    println!("{}", day01::capcha(vec![1, 1, 2, 2]));
    println!("{}", day01::capcha(vec![1, 1, 1, 1]));
    println!("{}", day01::capcha(vec![1, 2, 3, 4]));
    println!("{}", day01::capcha(vec![9, 1, 2, 1, 2, 1, 2, 9]));
}
