use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let input = input.get_original();

    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            let digits_1 = input[i + 4..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>();
            let num_1 = digits_1.parse::<u64>().ok()?;

            if input
                .chars()
                .nth(i + 4 + digits_1.len())
                .is_none_or(|c| c != ',')
            {
                return None;
            }

            let digits_2 = input[i + 4 + 1 + digits_1.len()..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>();
            let num_2 = digits_2.parse::<u64>().ok()?;

            if input
                .chars()
                .nth(i + 4 + digits_1.len() + 1 + digits_2.len())
                .is_none_or(|c| c != ')')
            {
                return None;
            }

            Some(num_1 * num_2)
        })
        .sum()
}

fn part_b(input: &Input) -> u64 {
    let input = input.get_original();

    let mut dos: Vec<usize> = input.match_indices("do()").map(|i| i.0).collect();
    dos.sort_unstable();

    let mut donts: Vec<usize> = input.match_indices("don't()").map(|i| i.0).collect();
    donts.sort_unstable();

    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            if donts
                .iter()
                .filter(|dont| **dont < i)
                .max()
                .is_some_and(|dont| {
                    dos.iter()
                        .filter(|do_| **do_ < i)
                        .max()
                        .is_none_or(|do_| *do_ < *dont)
                })
            {
                return None;
            }

            let digits_1 = input[i + 4..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>();
            let num_1 = digits_1.parse::<u64>().ok()?;

            if input
                .chars()
                .nth(i + 4 + digits_1.len())
                .is_none_or(|c| c != ',')
            {
                return None;
            }

            let digits_2 = input[i + 4 + 1 + digits_1.len()..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>();
            let num_2 = digits_2.parse::<u64>().ok()?;

            if input
                .chars()
                .nth(i + 4 + digits_1.len() + 1 + digits_2.len())
                .is_none_or(|c| c != ')')
            {
                return None;
            }

            Some(num_1 * num_2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_helpers::mk_test_input;

    use super::*;

    #[test]
    fn part_a_works() {
        let input = mk_test_input!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        );
        assert_eq!(part_a(&input), 161);
    }

    #[test]
    fn part_b_works() {
        let input = mk_test_input!(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        );
        assert_eq!(part_b(&input), 48);
    }
}
