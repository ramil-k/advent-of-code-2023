use std::fs::File;
use std::io::{BufReader, Read};

const NUMBERS: [(&str, isize); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

pub fn parse_input_line(str: &str) -> Option<usize> {
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    for (i, c) in str.chars().enumerate() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c as usize - '0' as usize);
            }
            last = Some(c as usize - '0' as usize);
        } else {
            for (string, digit) in NUMBERS.iter() {
                if str[i..].starts_with(string) {
                    if first.is_none() {
                        first = Some(*digit as usize);
                    }
                    last = Some(*digit as usize);
                    break;
                }
            }
        }
    }
    if first.is_none() || last.is_none() {
        return None;
    }
    Some(10 * first.unwrap() + last.unwrap())
}

pub fn parse_input(str: &str) -> usize {
    str.split('\n')
        .map(|line| parse_input_line(line))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum()
}

fn main() {
    let a = File::open("/Users/ramil/Projects/advents/ofcode-2023/adoc-01/src/input")
        .expect("Unable to open input file");
    let mut input = String::new();
    BufReader::new(a)
        .read_to_string(&mut input)
        .expect("Unable to read input file");
    println!("{}", parse_input(&input));
}

#[cfg(test)]
mod test {
    use crate::parse_input;
    use crate::parse_input_line;

    #[test]
    fn test_parse_input_line() {
        assert_eq!(parse_input_line("ab"), None);
        assert_eq!(parse_input_line("two1nine"), Some(29));
        assert_eq!(parse_input_line("eightwothree"), Some(83));
        assert_eq!(parse_input_line("abcone2threexyz"), Some(13));
        assert_eq!(parse_input_line("xtwone3four"), Some(24));
        assert_eq!(parse_input_line("4nineeightseven2"), Some(42));
        assert_eq!(parse_input_line("zoneight234"), Some(14));
        assert_eq!(parse_input_line("7pqrstsixteen"), Some(76));

        assert_eq!(parse_input_line("1abc2"), Some(12));
        assert_eq!(parse_input_line("pqr3stu8vwx"), Some(38));
        assert_eq!(parse_input_line("a1b2c3d4e5f"), Some(15));
        assert_eq!(parse_input_line("treb7uchet"), Some(77));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("ab"), 0);
        assert_eq!(
            parse_input(
                "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "
            ),
            281
        );
    }
}
