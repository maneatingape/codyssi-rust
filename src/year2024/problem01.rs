use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> i32 {
    let mut sorted = input.to_vec();
    sorted.sort_unstable();
    let skip = if sorted.len() > 20 { 20 } else { 2 };
    sorted[..sorted.len() - skip].iter().sum()
}

pub fn part3(input: &[i32]) -> i32 {
    input.iter().zip([1, -1].iter().cycle()).map(|(m, s)| m * s).sum()
}
