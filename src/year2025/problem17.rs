use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Position = (u32, u32);
type Cache = HashMap<Position, u128>;

pub struct Input {
    graph: HashMap<Position, Vec<Position>>,
    start: Position,
    end: Position,
    moves: Vec<u32>,
    limit: u32,
}

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut graph = HashMap::new();

    let end: u32 = prefix.iter_unsigned().nth(2).unwrap();
    for i in 0..end {
        graph.insert((1, i), vec![(1, i + 1)]);
    }

    let start = (1, 0);
    let end = (1, end);
    graph.insert(end, vec![]);

    for [id, start, end, from, to] in prefix.iter_unsigned().skip(3).chunk::<5>() {
        graph.entry((from, start)).and_modify(|e| e.push((id, start)));

        for i in start..end {
            graph.insert((id, i), vec![(id, i + 1)]);
        }

        graph.insert((id, end), vec![(to, end)]);
    }

    let moves: Vec<u32> = suffix.iter_unsigned().collect();
    let limit = *moves.iter().max().unwrap();

    Input { graph, start, end, moves, limit }
}

pub fn part1(input: &Input) -> u128 {
    let size = 1 + input.end.1 as usize;
    let moves: Vec<_> = input.moves.iter().map(|&m| m as usize).collect();

    let mut steps = vec![0; size];
    steps[0] = 1;

    for i in 0..size {
        for &j in &moves {
            if i + j < size {
                steps[i + j] += steps[i];
            }
        }
    }

    steps[size - 1]
}

pub fn part2(input: &Input) -> u128 {
    let cache = &mut HashMap::new();
    cache.insert(input.end, 1);

    dfs(input, cache, input.start)
}

pub fn part3(input: &Input) -> String {
    let cache = &mut HashMap::new();
    cache.insert(input.end, 1);

    dfs(input, cache, input.start);

    let target = 100000000000000000000000000000;
    let mut position = input.start;
    let mut rank = 1;
    let mut path = Vec::new();

    path.push(String::from("S1_0"));

    while position != input.end {
        let mut next: Vec<_> =
            next_steps(input, position).iter().map(|&next| (next, &cache[&next])).collect();
        next.sort_unstable();

        let mut i = 0;
        while i < next.len() - 1 && rank + next[i].1 <= target {
            rank += next[i].1;
            i += 1;
        }

        position = next[i].0;
        path.push(format!("S{}_{}", position.0, position.1));
    }

    path.join("-")
}

fn dfs(input: &Input, cache: &mut Cache, position: Position) -> u128 {
    if let Some(&result) = cache.get(&position) {
        return result;
    }

    let result = next_steps(input, position).iter().map(|&next| dfs(input, cache, next)).sum();
    cache.insert(position, result);
    result
}

fn next_steps(input: &Input, from: Position) -> HashSet<Position> {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();

    todo.push_back((from, 0));

    while let Some((position, distance)) = todo.pop_front() {
        if input.moves.contains(&distance) {
            seen.insert(position);
        }
        if distance < input.limit {
            input.graph[&position].iter().for_each(|&next| todo.push_back((next, distance + 1)));
        }
    }

    seen
}
