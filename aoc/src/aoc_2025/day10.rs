use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use challenges_input::Input;
use itertools::Itertools;
use z3::{Optimize, Solvable, ast::Int};

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

#[derive(Debug)]
struct Day10Input {
    lines: Vec<InputLine>,
}

#[derive(Debug)]
struct InputLine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn parse_line(line: &str) -> InputLine {
    let lights_start = 1usize;
    let lights_end = line.chars().position(|i| i == ']').unwrap();
    let buttons_start = lights_end + 2;
    let joltages_start = line.chars().position(|i| i == '{').unwrap() + 1;
    let buttons_end = joltages_start - 3;
    let lights = line[lights_start..lights_end]
        .chars()
        .map(|i| i == '#')
        .collect();
    let buttons = line[buttons_start..=buttons_end]
        .split_whitespace()
        .map(|i| {
            i[1..i.len() - 1]
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect();
    let joltages = line[joltages_start..line.len() - 1]
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    InputLine {
        lights,
        buttons,
        joltages,
    }
}

fn parse_input(input: Input) -> Day10Input {
    Day10Input {
        lines: input.lines().map(|i| parse_line(&i)).collect(),
    }
}

fn part_a(input: Input) -> u64 {
    let input = parse_input(input);
    let num_lines = input.lines.len();
    input
        .lines
        .into_iter()
        .enumerate()
        .map(|(l_idx, input)| dijkstra(&input))
        .sum()
}

fn dijkstra(input: &InputLine) -> u64 {
    let presses: Vec<usize> = vec![];
    let mut frontier: BinaryHeap<Reverse<(u64, Box<[bool]>)>> = BinaryHeap::new();
    frontier.push(Reverse((
        0,
        vec![false; input.lights.len()].into_boxed_slice(),
    )));
    let mut explored: HashMap<Box<[bool]>, u64> = HashMap::new();
    while let Some(v) = frontier.pop() {
        for i in &input.buttons {
            let mut new = v.0.1.clone();
            for l in i {
                new[*l] = !new[*l];
            }
            let cost = v.0.0 + 1;
            if new[..] == input.lights[..] {
                return cost;
            }
            if explored.get(&new).is_some_and(|v| *v <= cost) {
                continue;
            }
            explored.insert(new.clone(), cost);
            frontier.push(Reverse((cost, new)));
        }
    }
    u64::MAX
}

fn part_b(input: Input) -> u64 {
    let input = parse_input(input);
    let num_lines = input.lines.len();
    input
        .lines
        .into_iter()
        .map(|input| {
            let vars = (0..input.buttons.len())
                .map(|i| Int::fresh_const(&format!("button_{i}_value")))
                .collect_vec();
            let nonnegative_constraints = vars.iter().map(|i| i.ge(0));
            let joltage_constraints =
                input
                    .joltages
                    .into_iter()
                    .enumerate()
                    .map(|(j_idx, joltage)| {
                        let button_sum = input
                            .buttons
                            .iter()
                            .enumerate()
                            .filter(|i| i.1.contains(&j_idx))
                            .map(|(i, _)| &vars[i])
                            .fold(Int::from_u64(0), |acc, i| acc + i);
                        button_sum.eq(joltage as u64)
                    });
            let constraints = nonnegative_constraints.chain(joltage_constraints);
            let solver = Optimize::new();
            for constraint in constraints {
                solver.assert(&constraint);
            }
            let total = vars.iter().fold(Int::from_u64(0), |acc, i| acc + i);
            solver.minimize(&total);
            solver.check(&[]);
            total
                .read_from_model(&solver.get_model().unwrap(), false)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum()
}

aoc_helpers::mk_aoc_test!(
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    7,
    33
);
