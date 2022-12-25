pub fn main() {
    let x: u32 = include_str!("../input.txt")
        .lines()
        .map(|i| {find_match(i)})
        .map(|m| {find_score(m)})
        .sum();

    println!("{}", x);
}

fn find_match(input: &str) -> char {
    assert_eq!((input.len() % 2), 0);
    assert!(input.len() > 0);

    let items1 = &input[0..input.len()/2];
    let items2 = &input[input.len()/2..];

    for  c in items1.chars() {
        if items2.contains(c) {
            return c;
        }
    }

    panic!("No match found");
}

fn find_score(input: char) -> u32 {
    return if (input as u32) < ('a' as u32) {
        27 + input as u32 - 'A' as u32
    } else {
        1 + (input as u32) - ('a' as u32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_finding() {
        assert_eq!('a', find_match("abcdauio"));
    }

    #[test]
    #[should_panic]
    fn test_invalid_len() {
        find_match("abc");
    }

    #[test]
    #[should_panic]
    fn test_no_match() {
        find_match("abcdef");
    }

    #[test]
    fn test_lowercase_scores() {
        assert_eq!(1 , find_score('a'));
        assert_eq!(2 , find_score('b'));
        assert_eq!(26 , find_score('z'));
        assert_eq!(27 , find_score('A'));
        assert_eq!(28 , find_score('B'));
        assert_eq!(52 , find_score('Z'));
    }

}