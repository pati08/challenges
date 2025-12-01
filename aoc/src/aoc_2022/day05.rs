use std::str::FromStr;

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = false;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}
impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [num, from, to, ..] = s
            .chars()
            .filter(|i| !i.is_ascii_alphabetic())
            .collect::<String>()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect_vec()[..]
        else {
            return Err(());
        };
        Ok(Self { num, from, to })
    }
}

fn part_a(mut input: Input) -> String {
    let mut stacks = input
        .lines_consuming()
        .take_while(|l| l.contains("["))
        .collect_vec()
        .into_iter()
        .rev()
        .fold(vec![], |mut acc, item| {
            for (idx, c) in item.chars().enumerate() {
                if !c.is_ascii_alphabetic() {
                    continue;
                }
                let idx = (idx - 1) / 4;
                let should_add = (idx + 1).saturating_sub(acc.len());
                acc.extend(vec![vec![]; should_add]);
                acc[idx].push(c);
            }
            acc
        });
    input.skip(1);
    while let Some(instr) = input.next::<Instruction>() {
        for _ in 0..instr.num {
            let Some(c) = stacks[instr.from - 1].pop() else {
                continue;
            };
            stacks[instr.to - 1].push(c);
        }
    }
    stacks.into_iter().filter_map(|mut i| i.pop()).collect()
}

fn part_b(mut input: Input) -> String {
    let mut stacks = input
        .lines_consuming()
        .take_while(|l| l.contains("["))
        .collect_vec()
        .into_iter()
        .rev()
        .fold(vec![], |mut acc, item| {
            for (idx, c) in item.chars().enumerate() {
                if !c.is_ascii_alphabetic() {
                    continue;
                }
                let idx = (idx - 1) / 4;
                let should_add = (idx + 1).saturating_sub(acc.len());
                acc.extend(vec![vec![]; should_add]);
                acc[idx].push(c);
            }
            acc
        });
    input.skip(1);
    while let Some(instr) = input.next::<Instruction>() {
        let from = &mut stacks[instr.from - 1];
        let moved = from.drain(from.len() - instr.num..).collect_vec();
        let to = &mut stacks[instr.to - 1];
        to.extend(moved);
    }
    stacks.into_iter().filter_map(|mut i| i.pop()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        );
        assert_eq!(part_a(input), "CMZ");
    }

    #[test]
    fn test_part_b() {
        let input = mk_test_input!(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        );
        assert_eq!(part_b(input), "MCD");
    }
}
