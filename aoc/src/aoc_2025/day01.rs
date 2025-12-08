use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    input
        .lines()
        .map(|l| l[1..].parse::<i64>().unwrap() * (l.starts_with("R") as i64 * 2 - 1))
        .scan(50i64, |state, i| {
            *state = (*state + i).rem_euclid(100);
            Some((*state == 0) as u64)
        })
        .sum()
}

fn part_b(input: Input) -> u64 {
    input
        .lines()
        .map(|l| l[1..].parse::<i64>().unwrap() * (l.starts_with("R") as i64 * 2 - 1))
        .flat_map(|l| std::iter::repeat_n(l.signum(), l.unsigned_abs() as usize))
        .scan(50i64, |state, i| {
            *state = (*state + i + 100) % 100;
            Some((*state == 0) as u64)
        })
        .sum()
}

aoc_helpers::mk_aoc_test!(
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    3,
    6
);
