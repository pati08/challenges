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
        .map(|i| if i < 0 { 0u64 } else { 1u64 })
        .fold(0u64, |acc, i| {
            (acc << 1) + i
        });
    let epsilon = !gamma & (2u64.pow(input.lines().next().unwrap().len() as _) - 1);
    epsilon * gamma
}

fn part_b(input: &Input) -> u64 {
    // we still need these values for our later computations, but only as a 
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
        .map(|i| if i < 0 { 0u64 } else { 1u64 })
        .fold(0u64, |acc, i| {
            (acc << 1) + dbg!(i)
        });
    let epsilon = !gamma & (2u64.pow(input.lines().next().unwrap().len() as _) - 1);

    let gen_rating = rating(input, gamma, epsilon, false);
    let scrub_rating = rating(input, gamma, epsilon, true);
    dbg!(gen_rating) * dbg!(scrub_rating)
}

fn rating(input: &Input, gamma: u64, epsilon: u64, least: bool) -> u64 {
    let len = input.lines().next().unwrap().len() as u64;
    let mut nums = input.lines().map(|l| u64::from_str_radix(&l, 2).unwrap()).collect::<Vec<_>>();

    let most_common_bit = |nums: &[u64], pos: u64| {
        let mut acc = 0;
        for &num in nums {
            if num & (1 << pos) > 0 {
                acc += 1;
            } else {
                acc -= 1;
            }
        }

        let mut res = if acc < 0 { false } else { true };

        if least {
            res = !res;
        }

        res
    };

    for bitpos in (0..len).rev() {
        if nums.len() <= 1 {
            break;
        }

        let common = most_common_bit(&nums, bitpos);

        nums = nums.iter().copied().filter(|i| (*i & (1 << bitpos) > 0) == common).collect();

        println!("On bit {bitpos}:\n{}",
            nums.iter().map(|i| format!("{i:b}")).join(", ")
        );
    }

    nums[0]
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
    230
);
