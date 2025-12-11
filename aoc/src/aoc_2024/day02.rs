use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    input
        .lines()
        .filter(|l| {
            let mut last = 0;
            for pair in l
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .array_windows::<2>()
            {
                let diff = pair[1] - pair[0];
                if diff.abs() < 1 || diff.abs() > 3 || (last != 0 && diff * last < 0) {
                    return false;
                }
                last = diff;
            }
            true
        })
        .count() as u64
}

fn part_b(input: &Input) -> u64 {
    input
        .lines()
        .filter(|l| {
            let levels = l
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_safe_b(&levels)
        })
        .count() as u64
}
fn is_safe_b(all_levels: &[i32]) -> bool {
    for idx in 0..all_levels.len() {
        let mut levels = all_levels[0..idx].to_vec();
        levels.extend(&all_levels[idx + 1..]);

        let mut any_bad = false;

        let mut last = 0;
        for i in 0..(levels.len() - 1) {
            let diff = levels[i + 1] - levels[i];
            if diff.abs() < 1 || diff.abs() > 3 || (last != 0 && diff * last < 0) {
                any_bad = true;
                break;
            }
            last = diff;
        }
        if !any_bad {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use aoc_helpers::mk_test_input;

    use super::*;

    #[test]
    fn part_a_works() {
        let input = mk_test_input!(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        );
        assert_eq!(part_a(&input), 2);
    }

    #[test]
    fn part_b_works() {
        let input = mk_test_input!(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        );
        assert_eq!(part_b(&input), 4);
    }
}
