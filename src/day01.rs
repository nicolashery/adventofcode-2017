pub fn capcha(input: Vec<u32>) -> u32 {
    let len = input.len();
    if len == 0 {
        return 0;
    }

    let first = input[0];
    input.iter()
        .enumerate()
        .filter(|&(i, n)| {
            let next = if i == len - 1 { first } else { input[i+1] };
            *n == next
        })
        .map(|(_, n)| n)
        .sum()
}
