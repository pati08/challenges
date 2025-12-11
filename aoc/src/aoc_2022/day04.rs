use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    input
        .lines()
        .filter(|i| {
            let [e1, e2, ..] = i
                .split(',')
                .map(|e| {
                    let [s, e, ..] = e
                        .split('-')
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect_vec()[..]
                    else {
                        unreachable!()
                    };
                    (s, e)
                })
                .collect_vec()[..]
            else {
                unreachable!()
            };
            (e1.0 <= e2.0 && e1.1 >= e2.1) || (e2.0 <= e1.0 && e2.1 >= e1.1)
        })
        .count() as u64
}

fn part_b(input: Input) -> u64 {
    input
        .lines()
        .filter(|i| {
            let [e1, e2, ..] = i
                .split(',')
                .map(|e| {
                    let [s, e, ..] = e
                        .split('-')
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect_vec()[..]
                    else {
                        unreachable!()
                    };
                    (s, e)
                })
                .collect_vec()[..]
            else {
                unreachable!()
            };
            e1.1.max(e2.1) - e1.0.min(e2.0) < e1.1 - e1.0 + e2.1 - e2.0 + 1
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        );
        assert_eq!(part_a(input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = mk_test_input!(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        );
        assert_eq!(part_b(input), 4);
    }
}
