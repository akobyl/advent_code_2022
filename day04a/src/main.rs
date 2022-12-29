#[derive(Debug)]
struct Range {
    start: u8,
    end: u8,
}

pub fn main() {
    let x  = include_str!("../input.txt")
        .lines()
        .map(|line| {parse_line(line)})
        .map(|ranges|->bool {is_contained(&ranges)})
        .filter(|b| *b)
        .count();

    println!("{}", x);
}

fn is_contained(ranges: &Vec<Range>) -> bool {
    assert_eq!(2, ranges.len());
    let r1 = ranges.get(0).unwrap();
    let r2 = ranges.get(1).unwrap();

    (r1.start >= r2.start && r1.end <= r2.end) || (r1.start <= r2.start && r1.end >= r2.end)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_str() {
        let r1 = Range { start: 2, end: 4 };
        assert_eq!(r1, Range::from_str("2-4"));

        let r2 = Range {
            start: 112,
            end: 344,
        };
        assert_eq!(r2, Range::from_str("112-344"));
    }

    #[test]
    fn test_order_ranges() {
        let mut x = vec![
            Range { start: 10, end: 15 },
            Range { start: 9, end: 12 },
            Range { start: 10, end: 11 },
        ];
        let x_sorted = vec![
            Range { start: 9, end: 12 },
            Range { start: 10, end: 11 },
            Range { start: 10, end: 15 },
        ];
        x.sort();
        assert_eq!(x_sorted, x);
    }

    #[test]
    fn test_range_contained() {
        let x1 = vec![
            Range { start: 10, end: 15 },
            Range { start: 9, end: 12 },
        ];
        assert!(!is_contained(&x1));

        let x2 = vec![
            Range { start: 9, end: 12 },
            Range { start: 10, end: 15 },
        ];
        assert!(!is_contained(&x2));

        let x3 = vec![
            Range { start: 9, end: 12 },
            Range { start: 10, end: 11 },
        ];
        assert!(is_contained(&x3));

        let x4 = vec![
            Range { start: 10, end: 11 },
            Range { start: 9, end: 12 },
        ];
        assert!(is_contained(&x4));

        let x5 = vec![
            Range { start: 10, end: 11 },
            Range { start: 10, end: 11 },
        ];
        assert!(is_contained(&x5));

        let x6 = vec![
            Range { start: 10, end: 10 },
            Range { start: 10, end: 10 },
        ];
        assert!(is_contained(&x6));
    }

    #[test]
    fn is_contained2() {
        let x1 = vec![
            Range { start: 2, end: 95 },
            Range { start: 2, end: 96 },
        ];
        assert!(is_contained(&x1));

        let x2 = vec![
            Range { start: 2, end: 96 },
            Range { start: 2, end: 95 },
        ];
        assert!(is_contained(&x2));
    }

    #[test]
    fn is_contained3() {
        let x3 = vec![
            Range { start: 70, end: 70 },
            Range { start: 40, end: 69 },
        ];
        assert!(!is_contained(&x3));
    }

    #[test]
    fn test_row() {
        assert!(is_contained(&parse_line("67-84,66-87")));
    }
}
