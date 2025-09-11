use crate::util::parse::*;

type Input = (Box<dyn Fn(u64) -> u64>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let mut iter = input.iter_unsigned();

    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap() as u32;
    let rooms = iter.collect();

    (Box::new(move |n| a + b * n.pow(c)), rooms)
}

pub fn part1(input: &Input) -> u64 {
    let (pricing, rooms) = input;
    pricing(median(rooms))
}

pub fn part2(input: &Input) -> u64 {
    let (pricing, rooms) = input;
    pricing(rooms.iter().filter(|room| room.is_multiple_of(2)).sum())
}

pub fn part3(input: &Input) -> u64 {
    let (pricing, rooms) = input;
    *rooms.iter().filter(|&&room| pricing(room) < 15_000_000_000_000).max().unwrap()
}

fn median(rooms: &[u64]) -> u64 {
    let mut rooms = rooms.to_vec();
    rooms.sort_unstable();
    rooms[rooms.len() / 2]
}
