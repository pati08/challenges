use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn to_code(s: &str) -> u8 {
    match s {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!(),
    }
}

fn bonus(a: u8, b: u8) -> u8 {
    match (a, b) {
        (a, b) if a == b => 3,
        (1, 3) | (2, 1) | (3, 2) => 0,
        _ => 6,
    }
}

fn part_a(input: &Input) -> u64 {
    let codes = input.lines().map(|v| {
        let parts: Vec<_> = v.split_whitespace().collect();
        (to_code(parts[0]), to_code(parts[1]))
    });
    codes.map(|(a, b)| u64::from(b + bonus(a, b))).sum()
}

fn part_b(input: &Input) -> u64 {
    input
        .lines()
        .map(|v| {
            let parts = v.split_whitespace().collect_vec();
            (
                to_code(parts[0]),
                match parts[1] {
                    "X" => 0,
                    "Y" => 3,
                    _ => 6,
                },
            )
        })
        .map(|(a, res)| {
            let b = (1..=3).find(|v| bonus(a, *v) == res).unwrap();
            (a, b)
        })
        .map(|(a, b)| u64::from(b + bonus(a, b)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_bonus() {
        assert_eq!(bonus(1, 1), 3);
        assert_eq!(bonus(1, 2), 6);
        assert_eq!(bonus(1, 3), 0);

        assert_eq!(bonus(2, 1), 0);
        assert_eq!(bonus(2, 2), 3);
        assert_eq!(bonus(2, 3), 6);

        assert_eq!(bonus(3, 1), 6);
        assert_eq!(bonus(3, 2), 0);
        assert_eq!(bonus(3, 3), 3);
    }

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "A Y
B X
C Z"
        );
        assert_eq!(part_a(&input), 15);
        assert_eq!(part_a(&mk_test_input!("A Y")), 8);
        assert_eq!(part_a(&mk_test_input!("B X")), 1);
        assert_eq!(part_a(&mk_test_input!("C Z")), 6);
    }
}
