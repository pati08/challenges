use std::ops::RangeInclusive;

use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let fresh = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('-')
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| {
            let [s, e, ..] = &l[..] else {
                unreachable!();
            };
            *s..=*e
        })
        .collect::<Vec<_>>();
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .filter(|l| fresh.iter().any(|r| r.contains(&l.parse().unwrap())))
        .count() as u64
}

fn part_b(input: &Input) -> u64 {
    let fresh = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('-')
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| {
            let [s, e, ..] = &l[..] else {
                unreachable!();
            };
            *s..=*e
        });
    let nonoverlapping = fresh.fold(vec![], |mut acc: Vec<RangeInclusive<u64>>, r| {
        let overlapping = acc
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_idx, i)| overlaps(&r, i))
            .collect::<Vec<_>>();
        acc = acc
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !overlapping.iter().any(|j| j.0 == *i))
            .map(|(_, i)| i)
            .collect();
        let mut overlapping: Vec<_> = overlapping.into_iter().map(|i| i.1).collect();
        overlapping.push(r);
        acc.push(merge(&overlapping));
        acc
    });
    nonoverlapping
        .into_iter()
        .map(|i| i.end() - i.start() + 1)
        .sum::<u64>()
}

fn merge(ranges: &[RangeInclusive<u64>]) -> RangeInclusive<u64> {
    let start = ranges.iter().map(|i| *i.start()).min().unwrap();
    let end = ranges.iter().map(|i| *i.end()).max().unwrap();
    start..=end
}

fn overlaps(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    let x1 = *r1.start();
    let x2 = *r1.end();
    let y1 = *r2.start();
    let y2 = *r2.end();
    x1.max(y1) <= x2.min(y2)
}

aoc_helpers::mk_aoc_test!(
    &"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3,
    14
);
