use crate::util::iter::*;

pub fn parse(input: &str) -> Vec<bool> {
    input.lines().map(|line| line.as_bytes()[0] == b'T').collect()
}

pub fn part1(input: &[bool]) -> usize {
    input.iter().enumerate().filter_map(|(i, b)| b.then_some(i + 1)).sum()
}

pub fn part2(input: &[bool]) -> usize {
    input.iter().chunk::<2>().enumerate().filter(pair).count()
}

pub fn part3(input: &[bool]) -> usize {
    let mut gates = input.to_vec();
    let mut result = 0;

    while gates.len() > 1 {
        result += gates.iter().filter(|&&b| b).count();
        gates = gates.iter().chunk::<2>().enumerate().map(|t| pair(&t)).collect();
    }

    result
}

fn pair(&(i, [&a, &b]): &(usize, [&bool; 2])) -> bool {
    if i.is_multiple_of(2) { a && b } else { a || b }
}
