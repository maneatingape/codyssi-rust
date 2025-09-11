use codyssi::year2025::problem01::*;

const EXAMPLE: &str = "\
8
1
5
5
7
6
5
4
3
1
-++-++-++";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 21);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 23);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 189);
}
