use std::{
    collections::{HashMap, HashSet},
    hash::Hasher,
};

use challenges_input::Input;
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn sorted_pairs(
    input_hash: u64,
    mut pairs: Vec<((usize, usize), f32)>,
) -> Vec<((usize, usize), f32)> {
    pairs.par_sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap().reverse());
    pairs
}

fn part_a(input: &Input) -> u64 {
    let mut input = input.clone();
    let best_n: usize = input.next().unwrap();
    let coords: Vec<[f32; 3]> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect();
    let pairs = gen_pairs(&coords);
    let input_hash = input
        .lines()
        .fold(std::hash::DefaultHasher::new(), |mut acc, l| {
            acc.write(l.as_bytes());
            acc
        })
        .finish();
    let sorted = sorted_pairs(input_hash, pairs);
    let best_n_pairs = &sorted[..best_n];
    let map = best_n_pairs.iter().map(|i| i.0).fold(
        HashMap::<usize, Vec<usize>>::new(),
        |mut acc, (a, b)| {
            let ent_a = acc.entry(a).or_default();
            if !ent_a.contains(&b) {
                ent_a.push(b);
            }
            let ent_b = acc.entry(b).or_default();
            if !ent_b.contains(&a) {
                ent_b.push(a);
            }
            acc
        },
    );

    let mut subnets = subnets_sizes(&map, coords.len());
    subnets.sort_unstable();
    subnets.reverse();
    subnets[0..3].iter().product::<usize>() as u64
}

fn subnets_sizes(map: &HashMap<usize, Vec<usize>>, len: usize) -> Vec<usize> {
    let mut explored = vec![false; len];
    let start = map.keys().next().unwrap();

    let mut subnets = vec![];

    for i in 0..len {
        if explored[i] {
            continue;
        }
        let mut contents = vec![];
        let size = floodfill(i, map, &mut explored[..], &mut contents);
        subnets.push(size);
    }
    subnets
}

fn floodfill(
    start: usize,
    map: &HashMap<usize, Vec<usize>>,
    explored: &mut [bool],
    contents: &mut Vec<usize>,
) -> usize {
    if explored[start] {
        return 0;
    }
    contents.push(start);
    explored[start] = true;
    let mut sum = 1;
    let Some(connections) = map.get(&start) else {
        return sum;
    };
    for i in connections {
        sum += floodfill(*i, map, explored, contents);
    }
    sum
}

fn dist([x1, y1, z1]: [f32; 3], [x2, y2, z2]: [f32; 3]) -> f32 {
    f32::sqrt((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2))
}

fn gen_pairs(coords: &[[f32; 3]]) -> Vec<((usize, usize), f32)> {
    let mut pairs = HashMap::new();
    for (i, c1) in coords.iter().enumerate() {
        for (j, c2) in coords.iter().enumerate() {
            if i == j {
                continue;
            }
            let dist = dist(*c1, *c2);
            let ordered = ((i.min(j), i.max(j)), dist);

            pairs.entry(ordered.0).or_insert(ordered.1);
        }
    }
    pairs.drain().collect()
}

fn part_b(input: &Input) -> u64 {
    let mut input = input.clone();
    let _: usize = input.next().unwrap();
    let coords: Vec<[f32; 3]> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect();
    let pairs = gen_pairs(&coords);
    let input_hash = input
        .lines()
        .fold(std::hash::DefaultHasher::new(), |mut acc, l| {
            acc.write(l.as_bytes());
            acc
        })
        .finish();
    let sorted = sorted_pairs(input_hash, pairs.clone());
    let start_state: Vec<HashSet<usize>> = vec![];
    let length = coords.len();
    let pair = sorted
        .into_iter()
        .map(|i| i.0)
        .scan((false, start_state), |acc, (a, b)| {
            if acc.0 {
                return None;
            }
            let a_net = acc.1.iter().position(|i| i.contains(&a));
            let b_net = acc.1.iter().position(|i| i.contains(&b));
            match (a_net, b_net) {
                (None, None) => {
                    let mut set = HashSet::new();
                    set.insert(a);
                    set.insert(b);
                    acc.1.push(set);
                }
                (Some(set), None) => {
                    acc.1[set].insert(b);
                }
                (None, Some(set)) => {
                    acc.1[set].insert(a);
                }
                (Some(seta), Some(setb)) if seta != setb => {
                    let items = acc.1[setb].drain().collect_vec();
                    acc.1[seta].extend(items);
                    acc.1.remove(setb);
                }
                _ => (),
            }
            if acc.1.len() == 1 && acc.1[0].len() == length {
                acc.0 = true;
            }
            Some((a, b))
        })
        .last()
        .unwrap();
    let pair = (coords[pair.0], coords[pair.1]);
    u64::try_from(pair.0[0] as i64).unwrap() * u64::try_from(pair.1[0] as i64).unwrap()
}

aoc_helpers::mk_aoc_test!(
    &"10
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    40,
    25272
);
