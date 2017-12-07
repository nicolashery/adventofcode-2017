type Row = Vec<u32>;
type Sheet = Vec<Row>;

pub fn checksum(input: &str) -> u32 {
    let sheet = parse_input(input);

    sheet.iter().fold(0, |result, row| {
        let max = get_max(row);
        let min = get_min(row);
        let diff = max - min;
        result + diff
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
