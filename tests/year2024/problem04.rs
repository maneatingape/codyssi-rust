use codyssi::year2024::problem04::*;

const EXAMPLE: &str = "\
ADB <-> XYZ
STT <-> NYC
PLD <-> XYZ
ADB <-> NYC
JLI <-> NYC
PTO <-> ADB";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 6);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 15);
}
