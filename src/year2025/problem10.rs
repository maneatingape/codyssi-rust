use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    let modified: String = input.chars().filter(|&c| c != ' ').collect();
    Grid::parse(&modified)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    let size = input.width;
    let mut safest = u32::MAX;

    for i in 0..size {
        let mut row = 0;
        let mut col = 0;

        for j in 0..size {
            row += danger(input[Point::new(i, j)]);
            col += danger(input[Point::new(j, i)]);
        }

        safest = safest.min(row).min(col);
    }

    safest
}

pub fn part2(input: &Grid<u8>) -> u32 {
    dp(input, 15)
}

pub fn part3(input: &Grid<u8>) -> u32 {
    dp(input, input.width)
}

fn danger(b: u8) -> u32 {
    (b - b'0') as u32
}

fn dp(grid: &Grid<u8>, size: i32) -> u32 {
    let mut risk = grid.same_size_with(0);
    risk[ORIGIN] = danger(grid[ORIGIN]);

    for i in 1..size {
        risk[Point::new(i, 0)] = risk[Point::new(i - 1, 0)] + danger(grid[Point::new(i, 0)]);
        risk[Point::new(0, i)] = risk[Point::new(0, i - 1)] + danger(grid[Point::new(0, i)]);
    }

    for y in 1..size {
        for x in 1..size {
            risk[Point::new(x, y)] = risk[Point::new(x - 1, y)].min(risk[Point::new(x, y - 1)])
                + danger(grid[Point::new(x, y)]);
        }
    }

    risk[Point::new(size - 1, size - 1)]
}
