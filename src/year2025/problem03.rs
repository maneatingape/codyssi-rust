use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::Range;

type Input = Vec<Range<u32>>;

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<2>().map(|[start, end]| start..end).collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(count).sum()
}

pub fn part2(input: &Input) -> u32 {
    input.windows(2).step_by(2).map(merge).sum()
}

pub fn part3(input: &Input) -> u32 {
    input.windows(4).step_by(2).map(merge).max().unwrap()
}

fn count(range: &Range<u32>) -> u32 {
    range.end - range.start + 1
}

fn merge(ranges: &[Range<u32>]) -> u32 {
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable_by_key(|r| r.start);

    let mut current = ranges.remove(0);
    let mut boxes = 0;

    for next in ranges {
        if next.start > current.end {
            boxes += count(&current);
            current = next;
        } else {
            current.end = current.end.max(next.end);
        }
    }

    boxes + count(&current)
}
