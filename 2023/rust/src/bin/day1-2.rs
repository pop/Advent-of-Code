/// Your calculation isn't quite right. It looks like some of the digits are
/// actually spelled out with letters: one, two, three, four, five, six, seven,
/// eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last
/// digit on each line. For example:
///
/// ```
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
/// Adding these together produces 281.
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let result = read_to_string("inputs/day1.txt")?
        .lines()
        .fold(0, |acc, line| parse_line(line).unwrap() + acc);

    println!("{}", result);

    Ok(())
}

const PATTERNS: [&str; 20] = [
    "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
    "seven", "8", "eight", "9", "nine",
];

fn parse_line(line: &str) -> Result<u32, ParseError> {
    // Iterator over all numbers in this string
    let mut numbers: Vec<(usize, &str)> = PATTERNS
        .iter()
        .flat_map(|pat| line.match_indices(pat))
        .collect();

    let lowest: &str = numbers
        .iter()
        .min_by(|(x, _), (y, _)| x.cmp(y))
        .ok_or(ParseError::Invalid)
        .map(|(_, v)| *v)
        .map(parse_number)??;
    let highest: &str = numbers
        .iter()
        .max_by(|(x, _), (y, _)| x.cmp(y))
        .ok_or(ParseError::Invalid)
        .map(|(_, v)| *v)
        .map(parse_number)??;
    
    [lowest, highest].join("").parse::<u32>().map_err(|_| ParseError::Invalid)
}

fn parse_number(s: &str) -> Result<&str, ParseError> {
    match s {
        "0" | "zero" => Ok("0"),
        "1" | "one" => Ok("1"),
        "2" | "two" => Ok("2"),
        "3" | "three" => Ok("3"),
        "4" | "four" => Ok("4"),
        "5" | "five" => Ok("5"),
        "6" | "six" => Ok("6"),
        "7" | "seven" => Ok("7"),
        "8" | "eight" => Ok("8"),
        "9" | "nine" => Ok("9"),
        _ => Err(ParseError::Invalid),
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("two1nine"), Ok(29));
    assert_eq!(parse_line("eightwothree"), Ok(83));
    assert_eq!(parse_line("abcone2threexyz"), Ok(13));
    assert_eq!(parse_line("xtwone3four"), Ok(24));
    assert_eq!(parse_line("4nineeightseven2"), Ok(42));
    assert_eq!(parse_line("zoneight234"), Ok(14));
    assert_eq!(parse_line("7pqrstsixteen"), Ok(76));
}

#[derive(Debug, PartialEq)]
enum ParseError {
    Invalid,
}
