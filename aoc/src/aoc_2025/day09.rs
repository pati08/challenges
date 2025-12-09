use std::{collections::HashSet, io::Write};

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    let points = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();
    points
        .iter()
        .flat_map(|i| points.iter().map(move |j| (*i, *j)))
        .map(area)
        .max()
        .unwrap()
}

fn area(((x1, y1), (x2, y2)): ((u64, u64), (u64, u64))) -> u64 {
    (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1)
}

fn part_b(input: Input) -> u64 {
    let points = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();
    let mut allowed = HashSet::new();
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        add_to_allowed((a, b), &mut allowed);
    }
    fill_allowed(&mut allowed);
    points
        .iter()
        .flat_map(|i| points.iter().map(move |j| (*i, *j)))
        .filter(|a| is_allowed(*a, &allowed))
        .map(area)
        .max()
        .unwrap()
}

fn is_allowed(
    ((x1, y1), (x2, y2)): ((u64, u64), (u64, u64)),
    allowed: &HashSet<(u64, u64)>,
) -> bool {
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            if !allowed.contains(&(x, y)) {
                return false;
            }
        }
    }
    true
}

fn add_to_allowed(
    ((x1, y1), (x2, y2)): ((u64, u64), (u64, u64)),
    allowed: &mut HashSet<(u64, u64)>,
) {
    for x in x1.min(x2)..=x1.max(x2) {
        allowed.insert((x, y1));
        allowed.insert((x, y2));
    }
    for y in y1.min(y2)..=y1.max(y2) {
        allowed.insert((x1, y));
        allowed.insert((x2, y));
    }
}

fn fill_allowed(allowed: &mut HashSet<(u64, u64)>) {
    let max_x = allowed.iter().map(|i| i.0).max().unwrap();
    let max_y = allowed.iter().map(|i| i.0).max().unwrap();
    let mut contained = HashSet::new();
    const PROGRESS_BAR_SIZE: usize = 50;
    for y in 0..=max_y {
        let progress = (y as f32 / max_y as f32 * PROGRESS_BAR_SIZE as f32).round() as usize;
        print!(
            "\r[{}{}{}]",
            "=".repeat(progress),
            if progress < PROGRESS_BAR_SIZE {
                ">"
            } else {
                ""
            },
            " ".repeat(PROGRESS_BAR_SIZE - progress)
        );
        let _ = std::io::stdout().flush();
        let mut inside = false;
        let mut prev_edge = false;
        for x in 0..=max_x {
            let is_edge = allowed.contains(&(x, y));
            if is_edge && !prev_edge {
                inside = !inside;
            }
            if inside | is_edge {
                contained.insert((x, y));
            }
            prev_edge = is_edge;
        }
    }
    for i in contained {
        allowed.insert(i);
    }
}

aoc_helpers::mk_aoc_test!(
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    50,
    24
);
