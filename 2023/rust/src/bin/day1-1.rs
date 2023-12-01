/// On each line, the calibration value can be found by combining the first
/// digit and the last digit (in that order) to form a single two-digit number.
///
/// For example:
/// ```text
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are 12, 38, 15,
/// and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the
/// calibration values?
use std::{error::Error, fs::read_to_string, str::from_utf8};

fn main() -> Result<(), Box<dyn Error>> {
    let result = read_to_string("inputs/day1.txt")?
        .lines()
        .fold(0, |acc, line| parse_line(line).unwrap() + acc);

    println!("{}", result);

    Ok(())
}

fn parse_line(line: &str) -> Result<u32, ParseError> {
    // Iterator over all numbers in this string
    let mut numbers = line.chars().filter(|c| char::is_ascii_digit(c));

    // Parse the first number
    let first: char = numbers
        // Get the first number character
        .nth(0)
        // Map any errors to our ParseError
        .ok_or(ParseError::Invalid)?;

    // When we don't have a second number, duplicate the first
    let last: char = numbers
        .nth_back(0)
        .or(Some(first))
        .ok_or(ParseError::Invalid)?;

    // Coerce our [char, char] into a str
    from_utf8(&[first as u8, last as u8])
        .map_err(|_| ParseError::Invalid)?
        // Parse the str as a number
        .parse::<u32>()
        .map_err(|_| ParseError::Invalid)
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("1abc2"), Ok(12));
    assert_eq!(parse_line("pqr3stu8vwx"), Ok(38));
    assert_eq!(parse_line("a1b2c3d4e5f"), Ok(15));
    assert_eq!(parse_line("treb7uchet"), Ok(77));
    assert_eq!(parse_line("abcdefg"), Err(ParseError::Invalid));
}

#[derive(Debug, PartialEq)]
enum ParseError {
    Invalid,
}
