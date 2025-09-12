use codyssi::year2025::problem06::*;

const EXAMPLE: &str = "t#UD$%%DVd*L?^p?S$^@#@@$pF$?xYJ$LLv$@%EXO&$*iSFZuT!^VMHy#zKISHaBj?e*#&yRVdemc#?&#Q%j&ev*#YWRi@?mNQ@eK";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 59);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1742);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 2708);
}
