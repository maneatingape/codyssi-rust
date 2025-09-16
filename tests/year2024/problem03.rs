use codyssi::year2024::problem03::*;

const EXAMPLE: &str = "\
100011101111110010101101110011 2
83546306 10
1106744474 8
170209FD 16
2557172641 8
2B290C15 16
279222446 10
6541027340 8";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 78);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 3487996082);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), "30PzDC");
}
