use crate::util::parse::*;
use std::iter::once;

type Input = (Vec<u32>, Vec<usize>, usize);

pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.split("\n\n").collect();

    let frequencies = once(0).chain(chunks[0].iter_unsigned()).collect();
    let swaps = chunks[1].iter_unsigned().collect();
    let test_track = chunks[2].unsigned();

    (frequencies, swaps, test_track)
}

pub fn part1(input: &Input) -> u32 {
    let (mut frequencies, swaps, test_track) = input.clone();
    swaps.windows(2).step_by(2).for_each(|w| frequencies.swap(w[0], w[1]));
    frequencies[test_track]
}

pub fn part2(input: &Input) -> u32 {
    let (mut frequencies, mut swaps, test_track) = input.clone();
    swaps.push(swaps[0]);

    swaps.windows(3).step_by(2).for_each(|w| {
        let temp = frequencies[w[2]];
        frequencies[w[2]] = frequencies[w[1]];
        frequencies[w[1]] = frequencies[w[0]];
        frequencies[w[0]] = temp;
    });

    frequencies[test_track]
}

pub fn part3(input: &Input) -> u32 {
    let (mut frequencies, swaps, test_track) = input.clone();

    for w in swaps.windows(2).step_by(2) {
        let first = w[0].min(w[1]);
        let second = w[0].max(w[1]);
        let size = (second - first).min(frequencies.len() - second);

        (0..size).for_each(|offset| frequencies.swap(first + offset, second + offset));
    }

    frequencies[test_track]
}
