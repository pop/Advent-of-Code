/// As you continue your walk, the Elf poses a second question: in each game you played, what is
/// the fewest number of cubes of each color that could have been in the bag to make the game
/// possible?
///
/// Again consider the example games from earlier:
///
/// ```text
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// ```
///
/// * In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
///   If any color had even one fewer cube, the game would have been impossible.
/// * Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
/// * Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
/// * Game 4 required at least 14 red, 3 green, and 15 blue cubes.
/// * Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied
/// together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560,
/// 630, and 36, respectively. Adding up these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present. What is the sum of
/// the power of these sets?
use std::{cmp::Ordering, error::Error, fs::read_to_string, ops::Add};

fn main() -> Result<(), Box<dyn Error>> {
    let result = read_to_string("inputs/day2.txt")?
        .lines()
        .filter_map(|line| parse(line).ok())
        // Find the minimum number of red, green, and blue for each game
        .map(|(_, v)| Cubes {
            red: v.iter().max_by_key(|c| c.red).expect("Max red value").red,
            green: v
                .iter()
                .max_by_key(|c| c.green)
                .expect("Max green value")
                .green,
            blue: v
                .iter()
                .max_by_key(|c| c.blue)
                .expect("Max blue value")
                .blue,
        })
        // Calculate the power of each cube
        .map(|Cubes { red, green, blue }| red as usize * green as usize * blue as usize)
        // Sum the powers of the cubes
        .fold(0, |acc, i| acc + i);

    println!("{}", result);

    Ok(())
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Cubes {
    red: u8,
    green: u8,
    blue: u8,
}

impl Add for Cubes {
    type Output = Self;

    // Required method
    fn add(self, rhs: Self) -> Self {
        Cubes {
            red: self.red + rhs.red,
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
        }
    }
}

impl PartialOrd for Cubes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // All components of self are less than other
        if self.red < other.red && self.blue < other.blue && self.green < other.green {
            Some(Ordering::Less)
        // Any components of self are greater than other
        } else if self.red > other.red || self.blue > other.blue || self.green > other.green {
            Some(Ordering::Greater)
        // I guess they're equal?
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// Parse lines like this:
///
///     Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
///
///  Returning a 4-element tuple of the game # and the three results
fn parse(line: &str) -> Result<(usize, Vec<Cubes>), ParseError> {
    let id = {
        let substr = line
            .strip_prefix("Game ")
            .expect("Line starts with 'Game '");
        let idx = substr.find(":").expect("Expected ':'");
        substr[..idx].parse::<usize>().expect("Game #")
    };
    let picks: Vec<Cubes> = {
        fn parse_color(l: &str, color: &str) -> u8 {
            l.find(color)
                .map(|idx| {
                    l[idx - 3..idx - 1]
                        .strip_prefix(" ")
                        .unwrap_or(&l[idx - 3..idx - 1])
                        .parse::<u8>()
                })
                .unwrap_or(Ok(0))
                .unwrap_or(0)
        }
        line.split(":")
            .last()
            .expect("Game values")
            .split(";")
            .map(|l| Cubes {
                red: parse_color(l, "red"),
                green: parse_color(l, "green"),
                blue: parse_color(l, "blue"),
            })
            .collect()
    };
    Ok((id, picks))
}

#[derive(Debug, PartialEq)]
struct ParseError;

#[test]
fn test_parse() {}
