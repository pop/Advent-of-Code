
fn split_input(input: &str) -> Vec<usize> {
    let v: Vec<&str> = input.split(",").collect();
    v.into_iter().map(
        |num| {
            num.parse::<usize>().unwrap()
        }
    ).collect()
}

fn intcode(mut input: Vec<usize>) -> usize {
    let mut pos = 0;
    loop {
        println!("{:?} || {} || {}", input, pos, input[pos]);
        let curr = input[pos];
        match curr {
            1 => {
                let c: usize = input[pos+3];
                let a: usize = input[pos+1];
                let b: usize = input[pos+2];
                input[c] = input[a] + input[b]
            },
            2 => {
                let c: usize = input[pos+3];
                let a: usize = input[pos+1];
                let b: usize = input[pos+2];
                input[c] = input[a] * input[b]
            },
            99 => return input[0],
            _ => return 0,
        }
        pos = pos + 4
    }
}

pub fn solution(input: &str) -> usize {
    let input = split_input(input);
    intcode(input)
}
