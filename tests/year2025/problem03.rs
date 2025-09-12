use codyssi::year2025::problem03::*;

const EXAMPLE: &str = "\
8-9 9-10
7-8 8-10
9-10 5-10
3-10 9-10
4-8 7-9
9-10 2-7";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 43);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 35);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 9);
}
