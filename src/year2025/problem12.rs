use crate::util::parse::*;
use std::collections::VecDeque;

type Input<'a> = (Vec<Vec<i64>>, VecDeque<Vec<&'a str>>, Vec<&'a str>);

pub fn parse(input: &str) -> Input<'_> {
    let chunks: Vec<_> = input.split("\n\n").collect();

    let grid = chunks[0].lines().map(|line| line.iter_signed().collect()).collect();
    let instructions =
        chunks[1].lines().map(|line| line.split_ascii_whitespace().collect()).collect();
    let actions = chunks[2].lines().collect();

    (grid, instructions, actions)
}

pub fn part1(input: &Input<'_>) -> i64 {
    let (mut grid, instructions, _) = input.clone();

    for instruction in instructions {
        handle_instruction(&mut grid, &instruction);
    }

    score(&grid)
}

pub fn part2(input: &Input<'_>) -> i64 {
    let (mut grid, mut instructions, actions) = input.clone();
    let mut instruction = None;

    for action in actions {
        match action {
            "TAKE" => instruction = instructions.pop_front(),
            "CYCLE" => instructions.push_back(instruction.take().unwrap()),
            "ACT" => handle_instruction(&mut grid, &instruction.take().unwrap()),
            _ => unreachable!(),
        }
    }

    score(&grid)
}

pub fn part3(input: &Input<'_>) -> i64 {
    let (mut grid, mut instructions, actions) = input.clone();
    let mut instruction = None;

    for &action in actions.iter().cycle() {
        match action {
            "TAKE" => instruction = instructions.pop_front(),
            "CYCLE" => instructions.push_back(instruction.take().unwrap()),
            "ACT" => handle_instruction(&mut grid, &instruction.take().unwrap()),
            _ => unreachable!(),
        }
        if instructions.is_empty() && instruction.is_none() {
            break;
        }
    }

    score(&grid)
}

fn handle_instruction(grid: &mut [Vec<i64>], instruction: &[&str]) {
    match instruction {
        ["SHIFT", "ROW", index, _, amount] => {
            let index = to_index(index);
            let amount = to_usize(amount);

            grid[index].rotate_right(amount);
        }
        ["SHIFT", "COL", index, _, amount] => {
            let index = to_index(index);
            let amount = to_usize(amount);

            let mut col: Vec<_> = grid.iter().map(|row| row[index]).collect();
            col.rotate_right(amount);

            grid.iter_mut().zip(col).for_each(|(row, cell)| row[index] = cell);
        }
        [action, value, "ROW", index] => {
            let function = to_function(action, value);
            let index = to_index(index);

            grid[index].iter_mut().for_each(function);
        }
        [action, value, "COL", index] => {
            let function = to_function(action, value);
            let index = to_index(index);

            grid.iter_mut().map(|row| &mut row[index]).for_each(function);
        }
        [action, value, "ALL"] => {
            let function = to_function(action, value);

            grid.iter_mut().flat_map(|row| row.iter_mut()).for_each(function);
        }
        _ => unreachable!(),
    }
}

fn to_function(action: &str, value: &str) -> Box<dyn Fn(&mut i64)> {
    let value: i64 = value.signed();

    match action {
        "ADD" => Box::new(move |n| *n = (*n + value).rem_euclid(0x4000_0000)),
        "SUB" => Box::new(move |n| *n = (*n - value).rem_euclid(0x4000_0000)),
        "MULTIPLY" => Box::new(move |n| *n = (*n * value).rem_euclid(0x4000_0000)),
        _ => unreachable!(),
    }
}

fn score(grid: &[Vec<i64>]) -> i64 {
    let size = grid.len();

    let rows: i64 = grid.iter().map(|row| row.iter().sum()).max().unwrap();
    let cols: i64 = (0..size).map(|index| grid.iter().map(|row| row[index]).sum()).max().unwrap();

    rows.max(cols)
}

fn to_usize(s: &str) -> usize {
    s.unsigned()
}

fn to_index(s: &str) -> usize {
    to_usize(s) - 1
}
