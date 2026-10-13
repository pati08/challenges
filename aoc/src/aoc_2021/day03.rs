use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let gamma = input
        .lines()
        .fold(Vec::new(), |mut acc: Vec<i32>, l| {
            if acc.len() < l.len() {
                acc.resize(l.len(), 0);
            }

            for (i, c) in l.chars().enumerate() {
                if c == '1' {
                    acc[i] += 1;
                } else {
                    acc[i] -= 1;
                }
            }

            acc
        })
        .into_iter()
        .map(|i| if i < 0 { 0 } else { 1 });
}

fn part_b(input: &Input) -> u64 {
    0
}

aoc_helpers::mk_aoc_test!(
    &"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    198,
    0
);
