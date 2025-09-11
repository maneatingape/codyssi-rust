use codyssi::year2025::problem02::*;

const EXAMPLE: &str = "\
Function A: ADD 495
Function B: MULTIPLY 55
Function C: RAISE TO THE POWER OF 3

5219
8933
3271
7128
9596
9407
7005
1607
4084
4525
5496";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 9130674516975);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1000986169836015);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 5496);
}
