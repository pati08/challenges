use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let (mut l1, mut l2) = input
        .lines()
        .map(|l| l.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .map(|l| (l[0].parse::<u64>().unwrap(), l[1].parse::<u64>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();
    l1.sort_unstable();
    l2.sort_unstable();
    l1.into_iter().zip(l2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part_b(input: &Input) -> u64 {
    let (l1, l2) = input
        .lines()
        .map(|l| l.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .map(|l| (l[0].parse::<u64>().unwrap(), l[1].parse::<u64>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();
    l1.into_iter()
        .map(|i| i * l2.iter().filter(|&&j| j == i).count() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn part_a_works() {
        let input = mk_test_input!(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        );
        assert_eq!(part_a(&input), 11);
    }

    #[test]
    fn part_b_works() {
        let input = mk_test_input!(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        );
        assert_eq!(part_b(&input), 31);
    }
}
