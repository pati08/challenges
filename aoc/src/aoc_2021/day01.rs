use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    input
        .lines()
        .tuple_windows()
        .filter(|(a, b)| a.parse::<u32>().unwrap() < b.parse::<u32>().unwrap())
        .count() as u64
}

fn part_b(input: &Input) -> u64 {
    input
        .lines()
        .tuple_windows()
        .map(|(a, b, c)| {
            a.parse::<u32>().unwrap() + b.parse::<u32>().unwrap() + c.parse::<u32>().unwrap()
        })
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count() as u64
}

aoc_helpers::mk_aoc_test!(
    &"199
200
208
210
200
207
240
269
260
263",
    7,
    5
);
