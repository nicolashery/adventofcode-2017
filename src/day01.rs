pub fn captcha(input: &str) -> u32 {
  sum_captcha(parse_input(input), 1)
}

fn parse_input(input: &str) -> Vec<u32> {
  input.chars()
    .map(|c| {
      c.to_digit(10).expect(&format!("{} is not a digit (0-9)", c))
    })
    .collect()
}

fn sum_captcha(digits: Vec<u32>, step: usize) -> u32 {
    let len = digits.len();
    if len == 0 {
        return 0;
    }

    digits.iter()
        .enumerate()
        .filter(|&(i, n)| {
            let next_i = (i + step) % len;
            *n == digits[next_i]
        })
        .map(|(_, n)| n)
        .sum()
}
