pub fn captcha(input: &str) -> u32 {
  sum_captcha(parse_input(input))
}

fn parse_input(input: &str) -> Vec<u32> {
  input.chars()
    .map(|c| {
      c.to_digit(10).expect(&format!("{} is not a digit (0-9)", c))
    })
    .collect()
}

fn sum_captcha(digits: Vec<u32>) -> u32 {
    let len = digits.len();
    if len == 0 {
        return 0;
    }

    let first = digits[0];
    digits.iter()
        .enumerate()
        .filter(|&(i, n)| {
            let next = if i == len - 1 { first } else { digits[i+1] };
            *n == next
        })
        .map(|(_, n)| n)
        .sum()
}
