use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    input
        .lines()
        .map(|l| l[1..].parse::<i64>().unwrap() * if l.starts_with("R") { 1 } else { -1 })
        .scan(50i64, |state, i| {
            *state = (*state + i + 100) % 100;
            if *state == 0 {
                Some(true)
            } else {
                Some(false)
            }
        })
        .filter(|i| *i)
        .count() as u64
}

fn part_b(input: Input) -> u64 {
    input
        .lines()
        .map(|l| l[1..].parse::<i64>().unwrap() * if l.starts_with("R") { 1 } else { -1 })
        .flat_map(|l| {
            if l < 0 {
                std::iter::repeat_n(-1, l.abs() as usize)
            } else {
                std::iter::repeat_n(1, l.abs() as usize)
            }
        })
        .scan(50i64, |state, i| {
            *state = (*state + i + 100) % 100;
            if *state == 0 {
                Some(true)
            } else {
                Some(false)
            }
        })
        .filter(|i| *i)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn part_a_works() {
        let input = mk_test_input!(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        );
        assert_eq!(part_a(input), 3);
    }

    #[test]
    fn part_b_works() {
        let input = mk_test_input!(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        );
        assert_eq!(part_b(input), 6);
    }
}
