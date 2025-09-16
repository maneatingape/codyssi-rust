// Templates

// pub fn parse(_input: &str) -> Vec<u32> {
//     vec![]
// }
//
// pub fn part1(_input: &[u32]) -> u32 {
//     123
// }
//
// pub fn part2(_input: &[u32]) -> u32 {
//     456
// }
//
// pub fn part3(_input: &[u32]) -> u32 {
//     789
// }

// use codyssi::year2025::problem00::*;
//
// const EXAMPLE: &str = "";
//
// #[test]
// fn part1_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part1(&input), 123);
// }
//
// #[test]
// fn part2_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part2(&input), 456);
// }
//
// #[test]
// fn part3_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part3(&input), 789);
// }

macro_rules! test {
    ($year:tt $($problem:tt),*) => {
        pub mod $year {$(pub mod $problem;)*}
    }
}
test!(year2024
    problem01, problem02, problem03, problem04
);

test!(year2025
    problem01, problem02, problem03, problem04, problem05, problem06, problem07, problem08,
    problem09, problem10, problem11, problem12, problem13, problem14, problem15, problem16,
    problem17, problem18
);
