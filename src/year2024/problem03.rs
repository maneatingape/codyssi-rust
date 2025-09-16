use crate::util::iter::*;
use crate::util::parse::*;

type Input = (u32, usize);

pub fn parse(input: &str) -> Input {
    input.split_ascii_whitespace().chunk::<2>().fold((0, 0), |(p1, p2), [a, b]| {
        let radix = b.unsigned();
        let number = usize::from_str_radix(a, radix).unwrap();
        (p1 + radix, p2 + number)
    })
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

pub fn part3(input: &Input) -> String {
    let lut: Vec<_> =
        ('0'..='9').chain('A'..='Z').chain('a'..='z').chain(['!', '@', '#']).collect();

    let mut number = input.1;
    let mut result = String::new();

    while number > 0 {
        result.insert(0, lut[number % 65]);
        number /= 65;
    }

    result
}
