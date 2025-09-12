use std::collections::VecDeque;

use crate::util::iter::*;
use crate::util::parse::*;

type Input = (Vec<u32>, Vec<(usize, usize, u32)>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let balances = prefix.iter_unsigned().collect();
    let transfers = suffix
        .split_ascii_whitespace()
        .chunk::<6>()
        .map(|[_, from, _, to, _, amount]| {
            let from = to_index(from.as_bytes()[0]);
            let to = to_index(to.as_bytes()[0]);
            let amount = amount.unsigned();
            (from, to, amount)
        })
        .collect();

    (balances, transfers)
}

pub fn part1(input: &Input) -> u32 {
    let (mut balances, transfers) = input.clone();

    let loan = 1_000_000;
    balances.iter_mut().for_each(|b| *b += loan);

    for (from, to, amount) in transfers {
        balances[from] -= amount;
        balances[to] += amount;
    }

    balances.sort_unstable();
    balances.iter().rev().take(3).sum::<u32>() - 3 * loan
}

pub fn part2(input: &Input) -> u32 {
    let (mut balances, transfers) = input.clone();

    for (from, to, amount) in transfers {
        let actual = amount.min(balances[from]);
        balances[from] -= actual;
        balances[to] += actual;
    }

    balances.sort_unstable();
    balances.iter().rev().take(3).sum()
}

pub fn part3(input: &Input) -> u32 {
    let (mut balances, transfers) = input.clone();
    let mut debts = vec![VecDeque::new(); balances.len()];

    for (from, to, amount) in transfers {
        let actual = amount.min(balances[from]);
        balances[from] -= actual;

        if amount > actual {
            debts[from].push_back((amount - actual, to));
        }

        repay(&mut balances, &mut debts, to, actual);
    }

    balances.sort_unstable();
    balances.iter().rev().take(3).sum()
}

fn to_index(b: u8) -> usize {
    (b - b'A') as usize
}

fn repay(
    balances: &mut Vec<u32>,
    debts: &mut Vec<VecDeque<(u32, usize)>>,
    from: usize,
    amount: u32,
) {
    balances[from] += amount;

    while balances[from] > 0
        && let Some((debt, to)) = debts[from].pop_front()
    {
        let actual = debt.min(balances[from]);
        balances[from] -= actual;

        if debt > actual {
            debts[from].push_front((debt - actual, to));
        }

        repay(balances, debts, to, actual);
    }
}
