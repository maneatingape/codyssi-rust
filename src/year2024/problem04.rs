use crate::util::iter::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Input = HashMap<u32, Vec<u32>>;

pub fn parse(input: &str) -> Input {
    let mut graph = HashMap::new();

    for [from, _, to] in input.split_ascii_whitespace().map(to_u32).chunk::<3>() {
        graph.entry(from).or_insert(Vec::new()).push(to);
        graph.entry(to).or_insert(Vec::new()).push(from);
    }

    graph
}

pub fn part1(input: &Input) -> usize {
    input.len()
}

pub fn part2(input: &Input) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    let mut result = 0;

    let start = to_u32("STT");
    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((location, distance)) = todo.pop_front() {
        result += 1;

        if distance < 3 {
            for &next in &input[&location] {
                if seen.insert(next) {
                    todo.push_back((next, distance + 1));
                }
            }
        }
    }

    result
}

pub fn part3(input: &Input) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    let mut result = 0;

    let start = to_u32("STT");
    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((location, distance)) = todo.pop_front() {
        result += distance;

        for &next in &input[&location] {
            if seen.insert(next) {
                todo.push_back((next, distance + 1));
            }
        }
    }

    result
}

fn to_u32(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| (acc << 8) | b as u32)
}
