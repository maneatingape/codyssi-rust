use codyssi::util::ansi::*;
use codyssi::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::Path;

fn main() {
    // Parse command line options
    let args: Vec<_> = args().collect();
    let (year, problem) = if let Some(arg) = args.get(1) {
        let mut iter = arg.split("::");
        (iter.next(), iter.next())
    } else {
        (None, None)
    };

    // Filter solutions
    let solutions = empty()
        .chain(year2025())
        .filter(|solution| year.is_none_or(|y| y == solution.year))
        .filter(|solution| problem.is_none_or(|p| p == solution.problem));

    for Solution { year, problem, wrapper } in solutions {
        let path = Path::new("input").join(year).join(problem).with_extension("txt");

        if let Ok(data) = read_to_string(&path) {
            let (part1, part2, part3) = wrapper(data);

            println!("{YELLOW}{year} {problem}{RESET}");
            println!("    Part 1: {BOLD}{WHITE}{part1}{RESET}");
            println!("    Part 2: {BOLD}{WHITE}{part2}{RESET}");
            println!("    Part 3: {BOLD}{WHITE}{part3}{RESET}");
        } else {
            eprintln!("{BOLD}{RED}{year} {problem}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }
}

struct Solution {
    year: &'static str,
    problem: &'static str,
    wrapper: fn(String) -> (String, String, String),
}

macro_rules! run {
    ($year:tt $($problem:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let problem = stringify!($problem);
                let wrapper = |data: String| {
                    use $year::$problem::*;

                    let input = parse(&data);
                    let part1 = part1(&input).to_string();
                    let part2 = part2(&input).to_string();
                    let part3 = part3(&input).to_string();

                    (part1, part2, part3)
                };

                Solution { year, problem, wrapper }
            },)*]
        }
    }
}

run!(year2025
    problem01, problem02, problem03, problem04, problem05, problem06, problem07, problem08,
    problem09, problem10, problem11, problem12, problem13, problem14, problem15, problem16
);
