use codyssi::year2025::problem05::*;

const EXAMPLE: &str = "\
(-16, -191)
(92, 186)
(157, -75)
(39, -132)
(-42, 139)
(-74, -150)
(200, 197)
(-106, 105)";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 226);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 114);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 1384);
}
