extern crate adventofcode;

use adventofcode::day01::capcha;

#[test]
fn solve_1122() {
    assert_eq!(3, capcha(vec![1, 1, 2, 2]));
}

#[test]
fn solve_1111() {
    assert_eq!(4, capcha(vec![1, 1, 1, 1]));
}

#[test]
fn solve_1234() {
    assert_eq!(0, capcha(vec![1, 2, 3, 4]));
}

#[test]
fn solve_91212129() {
    assert_eq!(9, capcha(vec![9, 1, 2, 1, 2, 1, 2, 9]));
}
