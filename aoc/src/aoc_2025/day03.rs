use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn pos_max_first<T: PartialOrd>(iter: impl Iterator<Item = T>) -> Option<usize> {
    iter.enumerate()
        .fold(None, |acc, (idx, i)| {
            if acc.as_ref().is_none_or(|(_, v)| *v < i) {
                Some((idx, i))
            } else {
                acc
            }
        })
        .map(|v| v.0)
}

fn part_a(input: Input) -> u64 {
    input
        .lines()
        .map(|l| {
            let best_d1_pos = pos_max_first(l[..l.len() - 1].chars()).unwrap();
            let best_d2_pos =
                pos_max_first(l[best_d1_pos + 1..].chars()).unwrap() + best_d1_pos + 1;
            let best_d1 = l.chars().nth(best_d1_pos).unwrap().to_digit(10).unwrap() as u64;
            let best_d2 = l.chars().nth(best_d2_pos).unwrap().to_digit(10).unwrap() as u64;
            best_d1 * 10 + best_d2
        })
        .sum()
}

fn part_b(input: Input) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut joltage = 0;
            let mut start_cutoff = 0;
            for i in (0..12).rev() {
                let best_digit_pos =
                    pos_max_first(l[start_cutoff..l.len() - i].chars()).unwrap() + start_cutoff;
                let best_digit =
                    l.chars().nth(best_digit_pos).unwrap().to_digit(10).unwrap() as u64;
                joltage += best_digit * 10u64.pow(i as u32);
                start_cutoff = best_digit_pos + 1;
            }
            joltage
        })
        .sum()
}

aoc_helpers::mk_aoc_test!(
    "987654321111111
811111111111119
234234234234278
818181911112111",
    357,
    3121910778619
);
