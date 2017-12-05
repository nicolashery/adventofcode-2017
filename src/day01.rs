pub enum Mode {
  NextDigit,
  HalfwayAround,
}

pub fn captcha(input: &str, mode: Mode) -> u32 {
  let digits = parse_input(input);
  let step = match mode {
    Mode::NextDigit => 1,
    Mode::HalfwayAround => half_len(&digits),
  };
  sum_captcha(digits, step)
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

fn half_len(digits: &Vec<u32>) -> usize {
  let len = digits.len();
  if len % 2 != 0 {
    panic!("Expected list of length {} to contain an even number of elements", len);
  }

  len / 2
}
