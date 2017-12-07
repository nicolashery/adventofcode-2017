extern crate adventofcode;

use adventofcode::day02::checksum;

#[test]
fn basic_sheet() {
    assert_eq!(18, checksum("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8"));
}
