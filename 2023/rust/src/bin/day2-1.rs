/// ...As you walk, the Elf shows you a small bag and some cubes which are either red, green, or
/// blue. Each time you play this game, he will hide a secret number of cubes of each color in the
/// bag, and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
/// grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do
/// this a few times per game.
///
/// You play several games and record the information from each game (your puzzle input). Each game
/// is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated
/// list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// ```text
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// ```
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first
/// set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue
/// cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained only
/// 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded
/// with that configuration. However, game 3 would have been impossible because at one point the
/// Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because
/// the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have
/// been possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded with only 12 red
/// cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
use std::{cmp::Ordering, error::Error, fs::read_to_string, ops::Add};

fn main() -> Result<(), Box<dyn Error>> {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let limit = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = read_to_string("inputs/day2.txt")?
        .lines()
        .filter_map(|line| parse(line).ok())
        .filter_map(|(i, v)| {
            v.iter()
                .all(|c| c.red <= limit.red && c.blue <= limit.blue && c.green <= limit.green)
                .then_some(i)
        })
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
fn test_parse() {
    let cases = [
        (
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            (
                1,
                vec![
                    Cubes {
                        blue: 3,
                        red: 4,
                        ..Default::default()
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Cubes {
                        green: 2,
                        ..Default::default()
                    },
                ],
                true,
            ),
        ),
        (
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            (
                2,
                vec![
                    Cubes {
                        blue: 1,
                        green: 2,
                        ..Default::default()
                    },
                    Cubes {
                        green: 3,
                        blue: 4,
                        red: 1,
                    },
                    Cubes {
                        green: 1,
                        blue: 1,
                        ..Default::default()
                    },
                ],
                true,
            ),
        ),
        (
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            (
                3,
                vec![
                    Cubes {
                        green: 8,
                        blue: 6,
                        red: 20,
                    },
                    Cubes {
                        blue: 5,
                        red: 4,
                        green: 13,
                    },
                    Cubes {
                        green: 5,
                        red: 1,
                        ..Default::default()
                    },
                ],
                false,
            ),
        ),
        (
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            (
                4,
                vec![
                    Cubes {
                        green: 1,
                        red: 3,
                        blue: 6,
                    },
                    Cubes {
                        green: 3,
                        red: 6,
                        ..Default::default()
                    },
                    Cubes {
                        green: 3,
                        blue: 15,
                        red: 14,
                    },
                ],
                false,
            ),
        ),
        (
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            (
                5,
                vec![
                    Cubes {
                        red: 6,
                        blue: 1,
                        green: 3,
                    },
                    Cubes {
                        blue: 2,
                        red: 1,
                        green: 2,
                    },
                ],
                true,
            ),
        ),
    ];

    let limit = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    cases.iter().for_each(|(input, (idx, v, _))| {
        assert_eq!(parse(input), Ok((*idx, v.clone())));
    });
}
