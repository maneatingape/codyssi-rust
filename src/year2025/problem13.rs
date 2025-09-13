use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Edges = HashMap<u32, Vec<u32>>;
type Weights = HashMap<(u32, u32), u32>;
type Input = (Edges, Weights);

pub fn parse(input: &str) -> Input {
    let mut edges = HashMap::new();
    let mut weights = HashMap::new();

    for [from, _, to, _, weight] in input.split_ascii_whitespace().chunk::<5>() {
        let from = to_u32(from);
        let to = to_u32(to);
        let weight = weight.unsigned();

        edges.entry(from).or_insert(Vec::new()).push(to);
        edges.entry(to).or_insert(Vec::new());
        weights.insert((from, to), weight);
    }

    (edges, weights)
}

pub fn part1(input: &Input) -> u32 {
    bfs(input, false)
}

pub fn part2(input: &Input) -> u32 {
    bfs(input, true)
}

pub fn part3(input: &Input) -> u32 {
    let (edges, weights) = input;
    dfs(edges, weights, &mut Vec::new(), to_u32("STT"))
}

fn to_u32(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| (acc << 8) | b as u32)
}

pub fn bfs(input: &Input, use_weights: bool) -> u32 {
    let (edges, weights) = input;
    let start = to_u32("STT");

    let mut todo = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    let mut lengths = Vec::new();

    while let Some((position, length)) = todo.pop_front() {
        for &next in &edges[&position] {
            if seen.insert(next) {
                let extra = if use_weights { weights[&(position, next)] } else { 1 };
                let next_length = length + extra;

                todo.push_back((next, next_length));
                lengths.push(next_length);
            }
        }
    }

    lengths.sort_unstable();
    lengths.iter().rev().take(3).product()
}

fn dfs(edges: &Edges, weights: &Weights, path: &mut Vec<u32>, position: u32) -> u32 {
    let length = if let Some(index) = path.iter().position(|&node| node == position) {
        path.push(position);
        path[index..].windows(2).map(|w| weights[&(w[0], w[1])]).sum()
    } else {
        path.push(position);
        edges[&position].iter().map(|&next| dfs(edges, weights, path, next)).max().unwrap_or(0)
    };

    path.pop();
    length
}
