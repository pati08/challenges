use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(mut input: Input) -> u64 {
    input
        .next_line()
        .unwrap()
        .split(',')
        .flat_map(|v| {
            let [s, e, ..] = v
                .split('-')
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                unreachable!();
            };
            s..=e
        })
        .filter(|i| {
            let i = i.to_string();
            i[..i.len() / 2] == i[i.len() / 2..]
        })
        .sum()
}

fn is_repeat(s: &str) -> bool {
    'a: for i in 1..s.len() {
        let ss = &s[..i];
        if s.len().is_multiple_of(i) {
            continue;
        }
        for j in 1..(s.len() / i) {
            if ss != &s[j * i..(j + 1) * i] {
                continue 'a;
            }
        }
        return true;
    }
    false
}

fn part_b(mut input: Input) -> u64 {
    input
        .next_line()
        .unwrap()
        .split(',')
        .flat_map(|v| {
            let [s, e, ..] = v
                .split('-')
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                unreachable!();
            };
            s..=e
        })
        .filter(|i| is_repeat(&i.to_string()))
        .sum()
}

aoc_helpers::mk_aoc_test!(
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    1227775554,
    4174379265
);
