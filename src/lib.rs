macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util
    ansi, grid, heap, integer, iter, math, parse, point
);

library!(year2025
    problem01, problem02, problem03, problem04, problem05, problem06, problem07, problem08,
    problem09, problem10
);
