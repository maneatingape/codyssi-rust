use crate::util::parse::*;
use std::collections::HashMap;

type Input = Vec<(Rule, Vector)>;

#[derive(Clone, Copy)]
pub struct Rule {
    x: i32,
    y: i32,
    z: i32,
    a: i32,
    m: i32,
    r: i32,
}

impl Rule {
    fn from(c: &[i32]) -> Self {
        Rule { x: c[1], y: c[2], z: c[3], a: c[4], m: c[5], r: c[6] }
    }

    fn check(&self, v: &Vector) -> bool {
        let sum = self.x * v.x + self.y * v.y + self.z * v.z + self.a * (v.a - 1);
        sum.rem_euclid(self.m) == self.r
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
    a: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32, a: i32) -> Self {
        Vector { x, y, z, a }
    }

    fn from(c: &[i32]) -> Self {
        Vector { x: c[7], y: c[8], z: c[9], a: c[10] }
    }

    fn add(&self, other: &Vector, time: i32) -> Self {
        let x = (self.x + time * other.x).rem_euclid(10);
        let y = (self.y + time * other.y).rem_euclid(15);
        let z = (self.z + time * other.z).rem_euclid(60);
        let a = (self.a + time * other.a).rem_euclid(3);
        Vector { x, y, z, a }
    }
}

pub fn parse(input: &str) -> Input {
    let tokens: Vec<i32> = input.iter_signed().collect();
    tokens.chunks(11).map(|c| (Rule::from(c), Vector::from(c))).collect()
}

pub fn part1(input: &Input) -> usize {
    debris(input).len()
}

pub fn part2(input: &Input) -> i32 {
    bfs(input, 0)
}

pub fn part3(input: &Input) -> i32 {
    bfs(input, 3)
}

fn debris(input: &Input) -> Vec<(Vector, Vector)> {
    let mut debris = Vec::new();

    for &(rule, velocity) in input {
        for x in 0..10 {
            for y in 0..15 {
                for z in 0..60 {
                    for a in 0..3 {
                        let position = Vector::new(x, y, z, a);
                        if rule.check(&position) {
                            debris.push((position, velocity));
                        }
                    }
                }
            }
        }
    }

    debris
}

pub fn bfs(input: &Input, health: i32) -> i32 {
    let debris = debris(input);

    let xs = 0..10;
    let ys = 0..15;
    let zs = 0..60;
    let neighbours: [(i32, i32, i32); 7] =
        [(0, 0, 0), (1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

    let mut time = 0;
    let mut current = HashMap::new();
    let mut next = HashMap::new();

    current.insert((0, 0, 0), health);

    loop {
        time += 1;

        let mut space = [[[0; 60]; 15]; 10];
        for (p, v) in &debris {
            let m = p.add(v, time);
            if m.a == 1 {
                space[m.x as usize][m.y as usize][m.z as usize] += 1;
            }
        }
        space[0][0][0] = 0;

        for (&(x, y, z), &health) in &current {
            for (dx, dy, dz) in neighbours {
                let x = x + dx;
                let y = y + dy;
                let z = z + dz;

                if xs.contains(&x) && ys.contains(&y) && zs.contains(&z) {
                    let health = health - space[x as usize][y as usize][z as usize];
                    if health >= 0 {
                        if x == 9 && y == 14 && z == 59 {
                            return time;
                        }

                        let e = next.entry((x, y, z)).or_insert(health);
                        *e = (*e).max(health);
                    }
                }
            }
        }

        (current, next) = (next, current);
        next.clear();
    }
}
