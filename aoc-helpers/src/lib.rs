use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use challenges_input::Input;

#[macro_export]
macro_rules! mk_aoc_test {
    ($input:expr, $ans_a:expr, $ans_b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::mk_test_input;

            #[test]
            fn part_a_works() {
                let input = mk_test_input!($input);
                assert_eq!(part_a(input), $ans_a);
            }

            #[test]
            fn part_b_works() {
                let input = mk_test_input!($input);
                assert_eq!(part_b(input), $ans_b);
            }
        }
    };
}

#[macro_export]
macro_rules! days {
    ( $( $num:expr => $module:ident ),* ) => {
        $(
            mod $module;
        )*
        pub fn run(day: u8, problems_dir: std::path::PathBuf) -> String {
            #[allow(unused_variables)]
            match day {
                $(
                    $num => $module::run(aoc_helpers::get_input(&problems_dir, day, $module::TRIM)),
                )*
                _ => panic!("Day {} not implemented", day),
            }
        }
    };
}

#[macro_export]
macro_rules! years {
    ( $( $num:expr => $module:ident ),* ) => {
        $(
            mod $module;
        )*
        pub fn run(args: &Args) -> String {
            let year = args.year;
            let day = args.day;
            #[allow(unused_variables)]
            match year {
                $(
                    $num => $module::run(day, args.get_input_dir(year)),
                )*
                _ => panic!("Year {} not implemented", year),
            }
        }
    };
}

#[macro_export]
macro_rules! mk_test_input {
    ( $input:expr ) => {{
        challenges_input::Input::new($input.to_string(), super::TRIM)
    }};
}

pub fn run<A: Display + Send + 'static, B: Display + Send + 'static>(
    input: Input,
    part_a: impl FnOnce(Input) -> A + Send + 'static,
    part_b: impl FnOnce(Input) -> B + Send + 'static,
) -> String {
    let input_2 = input.clone();
    let h1 = std::thread::spawn(|| part_a(input_2));
    let h2 = std::thread::spawn(|| part_b(input));
    let a_res = h1.join().unwrap();
    let b_res = h2.join().unwrap();
    format!("Part A: {a_res}\nPart B: {b_res}")
}

#[derive(clap::Parser)]
pub struct Args {
    #[arg(short, long)]
    pub year: u16,
    /// The day number to run
    #[arg(short, long)]
    pub day: u8,
    #[arg(short, long)]
    input_dir: PathBuf,
}

impl Args {
    pub fn get_input_dir(&self, year: u16) -> PathBuf {
        let mut input_dir = self.input_dir.clone();
        input_dir.push(year.to_string());
        input_dir
    }
}

pub fn get_input(problems_dir: &Path, day: u8, trim: bool) -> Input {
    let problem_path = problems_dir.join(format!("day{day:02}.txt"));
    if !problem_path.is_file() {
        panic!("Input file not found: {problem_path:?}");
    }
    let raw = std::fs::read_to_string(&problem_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {problem_path:?}"));
    Input::new(raw, trim)
}

pub struct ProblemDesc;
impl challenges_input::ProblemDesc for ProblemDesc {
    type Args = Args;
    const TRIM: bool = false;
}
