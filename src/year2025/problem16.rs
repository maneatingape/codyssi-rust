use crate::util::iter::*;
use crate::util::parse::*;
use Instruction::*;
use Twist::*;

type Input = Vec<(Instruction, Twist)>;

#[derive(Clone, Copy)]
pub enum Instruction {
    Face(usize),
    Row(usize, usize),
    Col(usize, usize),
    Nop,
}

impl Instruction {
    fn from(line: &str) -> Self {
        match &line[..3] {
            "FAC" => {
                let value = line.iter_unsigned().next().unwrap();
                Face(value)
            }
            "ROW" => {
                let [row, value] = line.iter_unsigned().chunk::<2>().next().unwrap();
                Row(row, value)
            }
            "COL" => {
                let [row, value] = line.iter_unsigned().chunk::<2>().next().unwrap();
                Col(row, value)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Twist {
    Left,
    Right,
    Up,
    Down,
}

impl Twist {
    fn from(b: u8) -> Self {
        match b {
            b'L' => Left,
            b'R' => Right,
            b'U' => Up,
            b'D' => Down,
            _ => unreachable!(),
        }
    }

    fn next(self, i: usize, j: usize, k: usize) -> (usize, usize, usize) {
        match self {
            Left => (minus(k), j, i),
            Right => (k, j, minus(i)),
            Up => (i, minus(k), j),
            Down => (i, k, minus(j)),
        }
    }
}

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let first = prefix.lines().map(Instruction::from);
    let second = suffix.trim().bytes().map(Twist::from);

    first.zip(second.cycle()).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut i = 0;
    let mut j = 1;
    let mut k = 2;
    let mut absorptions = [0; 6];

    for (instruction, twist) in input {
        absorptions[k] += match instruction {
            Face(value) => 6400 * value,
            Row(_, value) | Col(_, value) => 80 * value,
            Nop => 0,
        };
        (i, j, k) = twist.next(i, j, k);
    }

    absorptions.sort_unstable();
    absorptions.iter().rev().take(2).product()
}

pub fn part2(input: &Input) -> u128 {
    perform(input)
}

pub fn part3(input: &Input) -> u128 {
    let mut extended = Vec::new();

    for &(instruction, twist) in input {
        match instruction {
            face @ Face(_) => extended.push((face, twist)),
            row @ Row(_, _) => {
                extended.extend_from_slice(&[(row, Left); 4]);
                extended.push((Nop, twist));
            }
            col @ Col(_, _) => {
                extended.extend_from_slice(&[(col, Up); 4]);
                extended.push((Nop, twist));
            }
            Nop => (),
        }
    }

    perform(&extended)
}

fn perform(input: &Input) -> u128 {
    const FACES: [(usize, usize); 6] = [(1, 2), (0, 2), (0, 1), (1, 2), (0, 2), (0, 1)];

    let mut i = 0;
    let mut j = 1;
    let mut k = 2;
    let mut faces = [[[0; 80]; 80]; 6];

    for &(instruction, twist) in input {
        let (x, y) = FACES[k];
        let face = &mut faces[k];

        match instruction {
            Face(value) => update_face(face, value),
            Row(index, value) => {
                if j == y {
                    update_row(face, index - 1, value);
                }
                if j == minus(y) {
                    update_row(face, 80 - index, value);
                }
                if j == x {
                    update_col(face, index - 1, value);
                }
                if j == minus(x) {
                    update_col(face, 80 - index, value);
                }
            }
            Col(index, value) => {
                if i == x {
                    update_col(face, index - 1, value);
                }
                if i == minus(x) {
                    update_col(face, 80 - index, value);
                }
                if i == y {
                    update_row(face, index - 1, value);
                }
                if i == minus(y) {
                    update_row(face, 80 - index, value);
                }
            }
            Nop => (),
        }

        (i, j, k) = twist.next(i, j, k);
    }

    faces.iter().map(dominant_sum).product()
}

fn update_face(face: &mut [[usize; 80]; 80], value: usize) {
    face.iter_mut().flat_map(|row| row.iter_mut()).for_each(|cell| *cell = (*cell + value) % 100);
}

fn update_row(face: &mut [[usize; 80]; 80], row: usize, value: usize) {
    face[row].iter_mut().for_each(|cell| *cell = (*cell + value) % 100);
}

fn update_col(face: &mut [[usize; 80]; 80], col: usize, value: usize) {
    face.iter_mut().for_each(|row| row[col] = (row[col] + value) % 100);
}

fn dominant_sum(face: &[[usize; 80]; 80]) -> u128 {
    let rows: usize = face.iter().map(|row| row.iter().sum()).max().unwrap();
    let cols: usize = (0..80).map(|col| face.iter().map(|row| row[col]).sum()).max().unwrap();
    (rows.max(cols) + 80) as u128
}

fn minus(i: usize) -> usize {
    (i + 3) % 6
}
