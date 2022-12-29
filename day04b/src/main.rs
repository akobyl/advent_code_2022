#[derive(Debug)]
struct Range {
    start: u8,
    end: u8,
}

pub fn main() {
    let x  = include_str!("../../day04a/input.txt")
        .lines()
        .map(|line| {parse_line(line)})
        .map(|ranges|->bool {is_overlap(&ranges)})
        .filter(|b| *b)
        .count();

    println!("{}", x);
}

fn is_overlap(ranges: &Vec<Range>) -> bool {
    assert_eq!(2, ranges.len());
    let r1 = ranges.get(0).unwrap();
    let r2 = ranges.get(1).unwrap();

    (r1.start <= r2.end) && (r2.start <= r1.end)
}

fn parse_line(line: &str) -> Vec<Range> {
    let ranges: Vec<Range> = line.split(",").map(|r| Range::from_str(r)).collect();
    assert_eq!(2, ranges.len());
    ranges
}

impl Range {
    fn from_str(input: &str) -> Range {
        let x: Vec<_> = input
            .split("-")
            .map(|c| c.parse::<u8>().unwrap())
            .collect();
        Range {
            start: *x.get(0).unwrap(),
            end: *x.get(1).unwrap(),
        }
    }
}
