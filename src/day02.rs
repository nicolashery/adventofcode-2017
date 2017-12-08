type Row = Vec<u32>;
type Sheet = Vec<Row>;

pub enum Mode {
    MinMaxDiff,
    EvenlyDivisible,
}

pub fn checksum(input: &str, mode: Mode) -> u32 {
    let sheet = parse_input(input);

    sheet.iter().fold(0, |result, row| {
        let row_result: u32 = match mode {
            Mode::MinMaxDiff => get_min_max_diff(row),
            Mode::EvenlyDivisible => get_evenly_divisible(row),
        };

        result + row_result
    })
}

fn parse_input(input: &str) -> Sheet {
    let v: Vec<Vec<u32>> = input
        .split("\n")
        .map(|r| {
            let row: Vec<u32> = r.split("\t")
                .map(|cell| {
                    let value: u32 = cell.parse().expect(&format!("{} is not an integer", cell));
                    value
                })
                .collect();
            row
        })
        .collect();

    v
}

fn get_min_max_diff(row: &Row) -> u32 {
    let max = get_max(row);
    let min = get_min(row);

    max - min
}

fn get_evenly_divisible(row: &Row) -> u32 {
    let len = row.len();

    for (i, &n) in row.iter().enumerate() {
        if i == len - 1 {
            break;
        }

        for &m in &row[i + 1..] {
            if n % m == 0 {
                return n / m;
            }
            if m % n == 0 {
                return m / n;
            }
        }
    }

    panic!("Could not find evenly divisible numbers in row");
}

fn get_max(row: &Row) -> &u32 {
    let len = row.len();
    if len == 0 {
        panic!("Cannot return max of empty row");
    }
    let first = &row[0];

    row.iter().fold(
        first,
        |max, n| if n > max { n } else { max },
    )
}

fn get_min(row: &Row) -> &u32 {
    let len = row.len();
    if len == 0 {
        panic!("Cannot return min of empty row");
    }
    let first = &row[0];

    row.iter().fold(
        first,
        |min, n| if n < min { n } else { min },
    )
}
