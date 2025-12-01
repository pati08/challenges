use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        c as u64 - 96
    } else {
        c as u64 - 64 + 26
    }
}

fn part_a(input: Input) -> u64 {
    input
        .lines()
        .map(|line| {
            let first_half = &line[..line.len() / 2];
            let second_half = &line[line.len() / 2..];
            let in_both = first_half
                .chars()
                .find(|c| second_half.contains(*c))
                .unwrap();
            priority(in_both)
        })
        .sum()
}

fn part_b(input: Input) -> u64 {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| {
            a.chars()
                .find(|ch| b.contains(*ch) && c.contains(*ch))
                .unwrap()
        })
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        );
        assert_eq!(part_a(input), 157);
    }
}
