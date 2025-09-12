use codyssi::year2025::problem08::*;

const EXAMPLE: &str = "\
tv8cmj0i2951190z5w44fe205k542l5818ds05ib425h9lj260ud38-l6a06
a586m0eeuqqvt5-k-8434hb27ytha3i75-lw23-0cj856l7zn8234a05eron";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 52);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 18);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 26);
}
