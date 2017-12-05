extern crate adventofcode;

use adventofcode::day01::captcha;

#[test]
fn solve_1122() {
    assert_eq!(3, captcha("1122"));
}

#[test]
fn solve_1111() {
    assert_eq!(4, captcha("1111"));
}

#[test]
fn solve_1234() {
    assert_eq!(0, captcha("1234"));
}

#[test]
fn solve_91212129() {
    assert_eq!(9, captcha("91212129"));
}
