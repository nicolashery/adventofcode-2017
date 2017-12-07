extern crate adventofcode;

use std::fs::File;
use std::io::prelude::*;

use adventofcode::day01;

fn main() {
    let day01_filname = "data/day01.txt";
    let mut day01_f =
        File::open(day01_filname).expect(&format!("File {} not found", day01_filname));
    let mut day01_input = String::new();
    day01_f.read_to_string(&mut day01_input).expect(&format!(
        "Something went wrong reading {}",
        day01_filname
    ));

    println!("==================== Day 01 ====================");
    println!(
        "Part One: {}",
        day01::captcha(&day01_input, day01::Mode::NextDigit)
    );
    println!(
        "Part Two: {}",
        day01::captcha(&day01_input, day01::Mode::HalfwayAround)
    );
    println!("");
}
