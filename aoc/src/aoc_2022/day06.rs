use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn all_unique<T: PartialEq>(list: &[T]) -> bool {
    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] == list[j] {
                return false;
            }
        }
    }
    true
}

fn part_a(input: &Input) -> u64 {
    input
        .get_original()
        .trim()
        .chars()
        .map_windows(|items: &[char; 4]| items.to_vec())
        .enumerate()
        .find_map(|(idx, items)| all_unique(&items).then_some(idx))
        .unwrap() as u64
        + 4
}

fn part_b(input: &Input) -> u64 {
    input
        .get_original()
        .trim()
        .chars()
        .map_windows(|items: &[char; 14]| items.to_vec())
        .enumerate()
        .find_map(|(idx, items)| all_unique(&items).then_some(idx))
        .unwrap() as u64
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_a(&input), 7);

        let input = mk_test_input!("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(part_a(&input), 5);

        let input = mk_test_input!("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(part_a(&input), 6);
    }

    #[test]
    fn test_part_b() {
        let input = mk_test_input!("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_b(&input), 19);

        let input = mk_test_input!("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(part_b(&input), 23);

        let input = mk_test_input!("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(part_b(&input), 23);
    }
}
