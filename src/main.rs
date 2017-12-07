extern crate adventofcode;

use std::fs::File;
use std::io::prelude::*;

use adventofcode::day01;
use adventofcode::day02;

fn main() {
    println!("==================== Day 01 ====================");
    let day01_input = read_input("data/day01.txt");
    println!(
        "Part One: {}",
        day01::captcha(&day01_input, day01::Mode::NextDigit)
    );
    println!(
        "Part Two: {}",
        day01::captcha(&day01_input, day01::Mode::HalfwayAround)
    );
    println!("");
    println!("");

    println!("==================== Day 02 ====================");
    let day02_input = read_input("data/day02.txt");
    println!("{}", day02::checksum(&day02_input));
    println!("");
    println!("");
}

fn read_input(filename: &str) -> String {
    let mut f = File::open(filename).expect(&format!("File {} not found", filename));
    let mut input = String::new();
    f.read_to_string(&mut input).expect(&format!(
        "Something went wrong reading {}",
        filename
    ));

    input
}
