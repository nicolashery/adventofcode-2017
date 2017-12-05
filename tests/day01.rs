extern crate adventofcode;

use adventofcode::day01::captcha;
use adventofcode::day01::Mode;

#[test]
fn next_digit_1122() {
    assert_eq!(3, captcha("1122", Mode::NextDigit));
}

#[test]
fn next_digit_1111() {
    assert_eq!(4, captcha("1111", Mode::NextDigit));
}

#[test]
fn next_digit_1234() {
    assert_eq!(0, captcha("1234", Mode::NextDigit));
}

#[test]
fn next_digit_91212129() {
    assert_eq!(9, captcha("91212129", Mode::NextDigit));
}

#[test]
fn halfway_around_1212() {
    assert_eq!(6, captcha("1212", Mode::HalfwayAround));
}

#[test]
fn halfway_around_1221() {
    assert_eq!(0, captcha("1221", Mode::HalfwayAround));
}

#[test]
fn halfway_around_123425() {
    assert_eq!(4, captcha("123425", Mode::HalfwayAround));
}

#[test]
fn halfway_around_123123() {
    assert_eq!(12, captcha("123123", Mode::HalfwayAround));
}

#[test]
fn halfway_around_12131415() {
    assert_eq!(4, captcha("12131415", Mode::HalfwayAround));
}
