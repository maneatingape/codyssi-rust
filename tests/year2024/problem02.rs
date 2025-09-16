use codyssi::year2024::problem02::*;

const EXAMPLE: &str = "\
TRUE
FALSE
TRUE
FALSE
FALSE
FALSE
TRUE
TRUE";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 19);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 7);
}
