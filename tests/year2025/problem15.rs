use codyssi::year2025::problem15::*;

const EXAMPLE: &str = "\
ozNxANO | 576690
pYNonIG | 323352
MUantNm | 422646
lOSlxki | 548306
SDJtdpa | 493637
ocWkKQi | 747973
qfSKloT | 967749
KGRZQKg | 661714
JSXfNAJ | 499862
LnDiFPd | 55528
FyNcJHX | 9047
UfWSgzb | 200543
PtRtdSE | 314969
gwHsSzH | 960026
JoyLmZv | 833936

MUantNm | 422646
FyNcJHX | 9047";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 12645822);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "ozNxANO-pYNonIG-MUantNm-lOSlxki-SDJtdpa-JSXfNAJ");
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), "pYNonIG");
}
