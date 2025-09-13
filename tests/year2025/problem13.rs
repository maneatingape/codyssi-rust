use codyssi::year2025::problem13::*;

const EXAMPLE: &str = "\
STT -> MFP | 5
AIB -> ZGK | 6
ZGK -> KVX | 20
STT -> AFG | 4
AFG -> ZGK | 16
MFP -> BDD | 13
BDD -> AIB | 5
AXU -> MFP | 4
CLB -> BLV | 20
AIB -> BDD | 13
BLV -> AXU | 17
AFG -> CLB | 2";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 36);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 44720);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 18);
}
