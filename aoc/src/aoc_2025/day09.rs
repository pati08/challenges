use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn part_a(input: Input) -> u64 {
    let points = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();
    points
        .iter()
        .flat_map(|i| points.iter().map(move |j| (*i, *j)))
        .map(area)
        .max()
        .unwrap()
}

fn area(((x1, y1), (x2, y2)): ((u64, u64), (u64, u64))) -> u64 {
    (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1)
}

fn part_b(input: Input) -> u64 {
    let points = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();
    let (points, x_map, y_map) = compress_coordinates(&points);
    let width = x_map.len() * 2 + 1;
    let height = y_map.len() * 2 + 1;
    let mut allowed = vec![false; width * height];
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        add_to_allowed((a, b), &mut allowed, width);
    }
    fill_allowed(&mut allowed, width);
    let best_rect = points
        .iter()
        .flat_map(|i| points.iter().map(move |j| (*i, *j)))
        .sorted_by_key(|i| area_decompressed(&x_map, &y_map, *i))
        .rev()
        .find(|i| is_allowed(*i, width, &allowed))
        .unwrap();
    area((
        decompress_coordinates(&x_map, &y_map, best_rect.0),
        decompress_coordinates(&x_map, &y_map, best_rect.1),
    ))
}

fn area_decompressed(
    x_map: &[u64],
    y_map: &[u64],
    (p1, p2): ((usize, usize), (usize, usize)),
) -> u64 {
    let p1 = decompress_coordinates(x_map, y_map, p1);
    let p2 = decompress_coordinates(x_map, y_map, p2);
    area((p1, p2))
}

fn decompress_coordinates(x_map: &[u64], y_map: &[u64], p: (usize, usize)) -> (u64, u64) {
    (x_map[(p.0 - 1) / 2], y_map[(p.1 - 1) / 2])
}

fn compress_coordinates(points: &[(u64, u64)]) -> (Vec<(usize, usize)>, Vec<u64>, Vec<u64>) {
    let all_xes = points.iter().map(|i| i.0).sorted().dedup().collect_vec();
    let all_yes = points.iter().map(|i| i.1).sorted().dedup().collect_vec();
    (
        points
            .iter()
            .map(|i| {
                (
                    all_xes.iter().position(|j| *j == i.0).unwrap() * 2 + 1,
                    all_yes.iter().position(|j| *j == i.1).unwrap() * 2 + 1,
                )
            })
            .collect(),
        all_xes,
        all_yes,
    )
}

fn fill_allowed(allowed: &mut [bool], width: usize) {
    let mut next = vec![0usize];
    let mut not_explored = vec![true; allowed.len()];

    while let Some(start) = next.pop() {
        if allowed[start] || !not_explored[start] {
            continue;
        }
        // allowed[start] = true;
        not_explored[start] = false;
        let (l, r, u, d) = (
            !start.is_multiple_of(width),
            start % width < width - 1,
            start > width,
            start + width < allowed.len(),
        );
        if l {
            if u {
                next.push(start - width - 1);
            }
            if d {
                next.push(start + width - 1);
            }
            next.push(start - 1);
        }
        if r {
            if u {
                next.push(start - width + 1);
            }
            if d {
                next.push(start + width + 1);
            }
            next.push(start + 1);
        }
        if u {
            next.push(start - width);
        }
        if d {
            next.push(start + width);
        }
    }
    for i in 0..not_explored.len() {
        allowed[i] |= not_explored[i];
    }
}

fn is_allowed(
    ((x1, y1), (x2, y2)): ((usize, usize), (usize, usize)),
    width: usize,
    allowed: &[bool],
) -> bool {
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            if !point_allowed(allowed, (x, y), width) {
                return false;
            }
        }
    }
    true
}

fn add_to_allowed(
    ((x1, y1), (x2, y2)): ((usize, usize), (usize, usize)),
    allowed: &mut [bool],
    width: usize,
) {
    for x in x1.min(x2)..=x1.max(x2) {
        allowed[x + y1 * width] = true;
        allowed[x + y2 * width] = true;
    }
    for y in y1.min(y2)..=y1.max(y2) {
        allowed[x1 + y * width] = true;
        allowed[x2 + y * width] = true;
    }
}

// fn fill_allowed(start: (u64, u64))

fn point_allowed(allowed: &[bool], point: (usize, usize), width: usize) -> bool {
    allowed[point.0 + point.1 * width]
}

aoc_helpers::mk_aoc_test!(
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    50,
    24
);
