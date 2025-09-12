use codyssi::year2025::problem11::*;

const EXAMPLE: &str = "\
32IED4E6L4 22
1111300022221031003013 4
1C1117A3BA88 13
1100010000010010010001111000000010001100101 2
7AJ5G2AB4F 22
k6IHxTD 61";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 9047685997827);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "4iWAbo%6");
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 2366);
}
