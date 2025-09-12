use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(' ').unwrap();
            let radix: usize = suffix.unsigned();
            prefix.bytes().fold(0, |acc, b| acc * radix + digit(b))
        })
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

pub fn part2(input: &[usize]) -> String {
    let lut: Vec<_> = ('0'..='9')
        .chain('A'..='Z')
        .chain('a'..='z')
        .chain(['!', '@', '#', '$', '%', '^'])
        .collect();

    let mut sum: usize = input.iter().sum();
    let mut result = String::new();

    while sum > 0 {
        result.insert(0, lut[sum % 68]);
        sum /= 68;
    }

    result
}

pub fn part3(input: &[usize]) -> usize {
    let sum: usize = input.iter().sum();
    let mut radix = sum.isqrt().isqrt();

    while radix.pow(4) < sum {
        radix += 1;
    }

    radix
}

fn digit(b: u8) -> usize {
    match b {
        b'0'..=b'9' => (b - b'0') as usize,
        b'A'..=b'Z' => (b - b'A' + 10) as usize,
        b'a'..=b'z' => (b - b'a' + 36) as usize,
        _ => unreachable!(),
    }
}
