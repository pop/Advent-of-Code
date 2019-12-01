use std::{
    io::{
        BufReader, BufRead,
    },
    fs::File,
    path::Path,
    error::Error,
};

type InputLines = std::io::Lines<std::io::BufReader<std::fs::File>>;

fn load_input(input: &str) -> InputLines {
    let path = Path::new(input);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    reader.lines()
}

#[allow(dead_code)]
fn f1(val: i64) -> i64 {
    let a: f64 = (val as f64) / 3.0;
    let b: f64 = a.floor();
    let c: f64 = b - 2.0;
    let d: i64 = c as i64;

    d
}

fn f2(val: i64) -> i64 {
    let a: f64 = (val as f64) / 3.0;
    let b: f64 = a.floor();
    let c: f64 = b - 2.0;
    let d: i64 = c as i64;

    if d > 0 {
        d + f2(d)
    } else {
        0
    }
}

fn f(val: i64) -> i64 {
    f2(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let inputs = [
            (12, 2),
            (14, 2),
            (1969, 654),
            (100756, 33583),

        ];
        for (val, expected) in inputs.into_iter() {
            assert_eq!(f1(*val), *expected);
        }
    }

    #[test]
    fn test_f2() {
        let inputs = [
            (14, 2),
            (1969, 966),
            (100756, 50346)
        ];

        for (val, expected) in inputs.into_iter() {
            assert_eq!(f2(*val), *expected);
        }
    }
}

pub fn solution(input: &str) -> i64 {
    let mut total = 0;

    for line in load_input(input) {
        let val = match line {
            Ok(l) => l.parse::<i64>().unwrap(),
            Err(_) => 0
        };
        if val > 0 {
            total += f(val)
        }
    }

    return total
}

