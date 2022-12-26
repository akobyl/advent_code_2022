use std::collections::HashSet;

pub fn main() {
    let x: Vec<_> = include_str!("../../day03a/input.txt")
        .lines().collect();

    let it: Vec<_> = x.chunks(3).collect();

    let mut sum: u32 = 0;
    for group in it {
        sum += find_score(find_card(group));
    }

    println!("{}", sum);
}

// fn find_card(input: &Vec<&str>) -> char {
fn find_card(input: &[&str]) -> char {
    assert_eq!(3, input.len());

    let a: HashSet<char> = input[0].chars().collect();
    let b: HashSet<char> = input[1]
        .chars()
        .filter(|c| { a.contains(c) })
        .collect();

    let c: Vec<_> = input[2]
        .chars()
        .filter(|c| { b.contains(c) })
        .collect();

    // assert_eq!(1, c.len());
    // If a common char is the one that is double, len() can be 2
    assert!(c.len() > 0);
    return *c.get(0).unwrap();
}

fn find_score(input: char) -> u32 {
    return if (input as u32) < ('a' as u32) {
        27 + input as u32 - 'A' as u32
    } else {
        1 + (input as u32) - ('a' as u32)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_card1() {
        let test = vec!["abcd", "ade", "afg"];
        assert_eq!('a', find_card(&test));
    }

    #[test]
    fn test_find_card2() {
        let test = vec!["adbcd", "eade", "afga"];
        assert_eq!('a', find_card(&test));
    }

    #[test]
    fn test_lowercase_scores() {
        assert_eq!(1, find_score('a'));
        assert_eq!(2, find_score('b'));
        assert_eq!(26, find_score('z'));
        assert_eq!(27, find_score('A'));
        assert_eq!(28, find_score('B'));
        assert_eq!(52, find_score('Z'));
    }
}
