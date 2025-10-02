use codyssi::util::ansi::*;
use codyssi::util::parse::*;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    // Parse command line options
    let mut iter = args().flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let (year, problem) = (iter.next(), iter.next());

    let solutions = [year2024(), year2025()];

    // Filter solutions then pretty print output.
    solutions
        .into_iter()
        .flatten()
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| problem.is_none_or(|p| p == s.problem))
        .for_each(|Solution { year, problem, wrapper }| {
            let path = format!("input/year{year}/problem{problem:02}.txt");

            if let Ok(data) = read_to_string(&path) {
                let (part1, part2, part3) = wrapper(&data);

                println!("{YELLOW}Year {year} Problem {problem}{RESET}");
                println!("    Part 1: {BOLD}{WHITE}{part1}{RESET}");
                println!("    Part 2: {BOLD}{WHITE}{part2}{RESET}");
                println!("    Part 3: {BOLD}{WHITE}{part3}{RESET}");
            } else {
                eprintln!("{BOLD}{RED}Year {year} Problem {problem}{RESET}");
                eprintln!("    Missing input!");
                eprintln!("    Place input file in {BOLD}{WHITE}{path}{RESET}");
            }
        });
}

struct Solution {
    year: u32,
    problem: u32,
    wrapper: fn(&str) -> (String, String, String),
}

macro_rules! run {
    ($year:tt $($problem:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).unsigned(),
                    problem: stringify!($problem).unsigned(),
                    wrapper: |data: &str| {
                        use codyssi::$year::$problem::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();
                        let part3 = part3(&input).to_string();

                        (part1, part2, part3)
                    }
                }
            ,)*]
        }
    }
}

run!(year2024
    problem01, problem02, problem03, problem04
);

run!(year2025
    problem01, problem02, problem03, problem04, problem05, problem06, problem07, problem08,
    problem09, problem10, problem11, problem12, problem13, problem14, problem15, problem16,
    problem17, problem18
);
