use std::str::FromStr;

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

#[allow(clippy::needless_pass_by_value)]
pub fn run(input: Input) -> String {
    format!("Part A: {}", part_a(&input))
}

struct Grid {
    width: usize,
    height: usize,
    blocks: Vec<usize>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_pos = s.chars().position(|c| c == 'x').unwrap();
        let colon_pos = s.chars().position(|c| c == ':').unwrap();
        let width = s[..x_pos].parse().unwrap();
        let height = s[x_pos + 1..colon_pos].parse().unwrap();
        let blocks = s[colon_pos + 2..]
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        Ok(Grid {
            width,
            height,
            blocks,
        })
    }
}

fn part_a(input: &Input) -> u64 {
    let sections = input.get_original().split("\n\n").collect_vec();
    sections
        .last()
        .unwrap()
        .lines()
        .map(|i| i.parse::<Grid>().unwrap())
        .filter(|i| {
            let blocks: usize = i.blocks.iter().sum();
            i.width * i.height >= 9 * blocks
        })
        .count() as u64
}
