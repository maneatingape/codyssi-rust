use crate::util::iter::*;
use crate::util::parse::*;
use std::iter::once;

type Input = (Vec<i32>, Vec<i32>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.trim().rsplit_once('\n').unwrap();

    let magnitudes = prefix.iter_signed().collect();
    let signs = suffix.bytes().map(|b| if b == b'+' { 1 } else { -1 }).collect();

    (magnitudes, signs)
}

pub fn part1(input: &Input) -> i32 {
    let magnitudes = input.0.iter().copied();
    let signs = input.1.iter().copied();
    zip(magnitudes, signs)
}

pub fn part2(input: &Input) -> i32 {
    let magnitudes = input.0.iter().copied();
    let signs = input.1.iter().rev().copied();
    zip(magnitudes, signs)
}

pub fn part3(input: &Input) -> i32 {
    let magnitudes = input.0.iter().chunk::<2>().map(|[tens, ones]| 10 * tens + ones);
    let signs = input.1.iter().rev().copied();
    zip(magnitudes, signs)
}

fn zip(magnitudes: impl Iterator<Item = i32>, signs: impl Iterator<Item = i32>) -> i32 {
    magnitudes.zip(once(1).chain(signs)).map(|(m, s)| m * s).sum()
}
