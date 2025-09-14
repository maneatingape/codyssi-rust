use crate::util::parse::*;
use std::cmp::Reverse;

#[derive(Clone, Copy)]
pub struct Item {
    quality: u32,
    cost: usize,
    unique: u32,
}

impl Item {
    fn from(chunk: &[&str]) -> Self {
        let quality = chunk[5].unsigned();
        let cost = chunk[8].unsigned();
        let unique = chunk[12].unsigned();
        Item { quality, cost, unique }
    }
}

pub fn parse(input: &str) -> Vec<Item> {
    let tokens: Vec<_> = input.split_ascii_whitespace().collect();
    tokens.chunks_exact(13).map(Item::from).collect()
}

pub fn part1(input: &[Item]) -> u32 {
    let mut items = input.to_vec();
    items.sort_unstable_by_key(|item| (item.quality, item.cost));
    items.iter().rev().take(5).map(|item| item.unique).sum()
}

pub fn part2(input: &[Item]) -> u32 {
    knapsack::<31>(input)
}

pub fn part3(input: &[Item]) -> u32 {
    knapsack::<301>(input)
}

pub fn knapsack<const W: usize>(items: &[Item]) -> u32 {
    let size = items.len();
    let mut table = vec![[(0, Reverse(0)); W]; size];

    for i in 1..size {
        for j in 1..W {
            let item = items[i];
            let previous = table[i - 1][j];

            table[i][j] = if item.cost > j {
                previous
            } else {
                let (quality, Reverse(unique)) = table[i - 1][j - item.cost];
                let current = (quality + item.quality, Reverse(unique + item.unique));
                if current > previous { current } else { previous }
            }
        }
    }

    let (quality, Reverse(unique)) = table[size - 1][W - 1];
    quality * unique
}
