pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> usize {
    input.iter().map(letters).sum()
}

pub fn part2(input: &[&[u8]]) -> usize {
    input
        .iter()
        .map(|line| {
            let start = line.len() / 10;
            let end = line.len() - start;
            let removed = end - start;

            letters(&&line[..start]) + letters(&&line[end..]) + digits(removed)
        })
        .sum()
}

pub fn part3(input: &[&[u8]]) -> usize {
    input
        .iter()
        .map(|original| {
            let mut compressed = original.to_vec();
            compressed.dedup();

            let mut units = letters(&compressed.as_slice());
            let mut start = 0;

            for c in compressed {
                let count = original.iter().skip(start).take_while(|&&b| b == c).count();
                units += digits(count);
                start += count;
            }

            units
        })
        .sum()
}

fn letters(line: &&[u8]) -> usize {
    line.iter().map(|&b| (b - b'A' + 1) as usize).sum()
}

fn digits(mut n: usize) -> usize {
    let mut units = 0;

    while n > 0 {
        units += n % 10;
        n /= 10;
    }

    units
}
