use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let (a, b) = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            (l.next().unwrap().to_owned(), l.next().unwrap().to_owned())
        })
        .map(|(d, v)| {
            let v: i32 = v.parse().unwrap();
            match d.as_str() {
                "forward" => (v, 0),
                "up" => (0, -v),
                "down" => (0, v),
                _ => unreachable!(),
            }
        })
        .fold((0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1));
    a as u64 * b as u64
}

fn part_b(input: &Input) -> u64 {
    let (a, b, _) = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            (l.next().unwrap().to_owned(), l.next().unwrap().to_owned())
        })
        .fold((0, 0, 0), |mut acc, (d, v)| {
            let v: i32 = v.parse().unwrap();
            match d.as_str() {
                "forward" => {
                    acc.0 += v;
                    acc.1 += v * acc.2;
                }
                "up" => acc.2 -= v,
                "down" => acc.2 += v,
                _ => unreachable!(),
            }
            acc
        });
    a as u64 * b as u64
}

aoc_helpers::mk_aoc_test!(
    &"forward 5
down 5
forward 8
up 3
down 8
forward 2",
    150,
    900
);
