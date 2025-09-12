pub fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .bytes()
        .map(|b| match b {
            b'a'..=b'z' => (b - b'a' + 1) as i32,
            b'A'..=b'Z' => (b - b'A' + 27) as i32,
            _ => 0,
        })
        .collect()
}

pub fn part1(input: &[i32]) -> usize {
    input.iter().filter(|&&v| v > 0).count()
}

pub fn part2(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part3(input: &[i32]) -> i32 {
    let mut current = 0_i32;
    let mut result = 0;

    for &next in input {
        current = if next == 0 { (current * 2 - 6).rem_euclid(52) + 1 } else { next };
        result += current;
    }

    result
}
