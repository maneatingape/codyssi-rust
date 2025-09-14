use codyssi::year2025::problem17::*;

const EXAMPLE: &str = "\
S1 : 0 -> 6 : FROM START TO END
S2 : 2 -> 3 : FROM S1 TO S1

Possible Moves : 1, 3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 6);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 17);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), "S1_0-S2_2-S2_3-S1_5-S1_6");
}
