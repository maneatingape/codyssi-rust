use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<Point> {
    input.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y)).collect()
}

pub fn part1(input: &[Point]) -> i32 {
    let distances: Vec<_> = input.iter().map(|p| p.manhattan(ORIGIN)).collect();
    *distances.iter().max().unwrap() - *distances.iter().min().unwrap()
}

pub fn part2(input: &[Point]) -> i32 {
    let start = *input.iter().min_by_key(|p| (p.manhattan(ORIGIN), p.x, p.y)).unwrap();
    let next = *input
        .iter()
        .filter(|&&p| p != start)
        .min_by_key(|p| (p.manhattan(start), p.x, p.y))
        .unwrap();

    next.manhattan(start)
}

pub fn part3(input: &[Point]) -> i32 {
    let mut islands = input.to_vec();
    let mut distance = 0;
    let mut current = ORIGIN;

    while !islands.is_empty() {
        let (index, _) = islands
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| (p.manhattan(current), p.x, p.y))
            .unwrap();

        let next = islands.remove(index);
        distance += current.manhattan(next);
        current = next;
    }

    distance
}
