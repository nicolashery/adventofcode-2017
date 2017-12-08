extern crate adventofcode;

use adventofcode::day02::checksum;
use adventofcode::day02::Mode;

#[test]
fn min_max_diff() {
    assert_eq!(
        18,
        checksum("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8", Mode::MinMaxDiff)
    );
}

#[test]
fn evenly_divisible() {
    assert_eq!(
        9,
        checksum("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5", Mode::EvenlyDivisible)
    );
}
