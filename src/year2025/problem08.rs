pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    input.iter().map(|line| line.bytes().filter(u8::is_ascii_alphabetic).count()).sum()
}

pub fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            remove(line, |a, b| a.is_ascii_digit() && (b.is_ascii_alphabetic() || b == b'-'))
        })
        .sum()
}

pub fn part3(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| remove(line, |a, b| a.is_ascii_digit() && b.is_ascii_alphabetic()))
        .sum()
}

fn remove(line: &str, pair: fn(u8, u8) -> bool) -> usize {
    let mut stack: Vec<u8> = Vec::new();

    for b in line.bytes() {
        if let Some(&head) = stack.last()
            && (pair(head, b) || pair(b, head))
        {
            stack.pop();
        } else {
            stack.push(b);
        }
    }

    stack.len()
}
