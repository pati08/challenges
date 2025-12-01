use itertools::Itertools;

use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    let input = input.get_original().split("\n\n").map(str::lines);
    let group_lines = input.map(|lines| {
        let lines = lines.map(|line| line.trim().parse::<u64>().unwrap());
        lines.sum::<u64>()
    });
    group_lines.max().unwrap()
}

fn part_b(input: Input) -> u64 {
    let input = input.get_original().split("\n\n").map(str::lines);
    let group_lines = input.map(|lines| {
        let lines = lines.map(|line| line.trim().parse::<u64>().unwrap());
        lines.sum::<u64>()
    });
    group_lines.sorted().rev().take(3).sum()
}
