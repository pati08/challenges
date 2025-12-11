use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

#[derive(Default)]
struct State {
    beam_cols: Vec<usize>,
    splits: usize,
}

#[derive(Default)]
struct StateB {
    beam_cols: Vec<(usize, usize)>,
    splits: usize,
}

fn part_a(input: &Input) -> u64 {
    input
        .lines()
        .fold(State::default(), |mut state, line| {
            for (col, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    state.beam_cols.push(col);
                } else if ch == '^' {
                    let mut new_beam_cols = state
                        .beam_cols
                        .clone()
                        .into_iter()
                        .flat_map(|i| {
                            if i == col {
                                vec![i - 1, i + 1]
                            } else {
                                vec![i]
                            }
                        })
                        .collect_vec();
                    let splits = new_beam_cols.len() - state.beam_cols.len();
                    new_beam_cols.sort_unstable();
                    new_beam_cols.dedup();
                    state.splits += splits;
                    state.beam_cols = new_beam_cols;
                }
            }
            state
        })
        .splits as u64
}

fn part_b(input: &Input) -> u64 {
    input
        .lines()
        .fold(StateB::default(), |mut state, line| {
            for (col, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    state.beam_cols.push((col, 1));
                    continue;
                }
                if ch != '^' {
                    continue;
                }
                let mut new_beam_cols = state
                    .beam_cols
                    .clone()
                    .into_iter()
                    .flat_map(|(i, c)| {
                        if i == col {
                            vec![(i - 1, c), (i + 1, c)]
                        } else {
                            vec![(i, c)]
                        }
                    })
                    .collect_vec();
                new_beam_cols.sort_unstable_by_key(|i| i.0);
                let new_beam_cols = custom_dedup(new_beam_cols.into_iter());
                let splits = new_beam_cols.iter().map(|i| i.1).sum::<usize>()
                    - state.beam_cols.iter().map(|i| i.1).sum::<usize>();
                state.splits += splits;
                state.beam_cols = new_beam_cols;
            }
            state
        })
        .splits as u64
        + 1
}

fn custom_dedup<T: PartialEq>(iter: impl Iterator<Item = (T, usize)>) -> Vec<(T, usize)> {
    iter.fold(vec![], |mut acc, i| {
        if let Some(v) = acc.last_mut()
            && v.0 == i.0
        {
            v.1 += i.1;
        } else {
            acc.push(i);
        }
        acc
    })
}

aoc_helpers::mk_aoc_test!(
    &".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    21,
    40
);
