use std::collections::{HashMap, HashSet};

use challenges_input::Input;

pub const TRIM: bool = true;
pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

#[derive(PartialEq, Clone)]
enum GridPos {
    None,
    Block,
}

const fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}
const fn next_pos(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}
const fn is_in_bounds(pos: (i32, i32), rows: usize, cols: usize) -> bool {
    pos.0 >= 0 && pos.0 < rows as i32 && pos.1 >= 0 && pos.1 < cols as i32
}

fn part_a(input: Input) -> u64 {
    let mut grid: Vec<Vec<GridPos>> = vec![];
    let mut guard = None;
    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::with_capacity(line.len()));
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => grid.last_mut().unwrap().push(GridPos::None),
                '#' => grid.last_mut().unwrap().push(GridPos::Block),
                '^' => {
                    guard = Some((i, j));
                    grid.last_mut().unwrap().push(GridPos::None);
                }
                _ => panic!("Unexpected character"),
            }
        }
    }
    let mut guard = guard.unwrap();

    let mut unique_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut dir = (-1i32, 0i32);

    unique_positions.insert(guard);

    while is_in_bounds(
        next_pos((guard.0 as i32, guard.1 as i32), dir),
        grid.len(),
        grid[0].len(),
    ) {
        let new_pos = next_pos((guard.0 as i32, guard.1 as i32), dir);
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if grid[new_pos.0][new_pos.1] == GridPos::Block {
            dir = turn_right(dir);
            continue;
        }
        guard = new_pos;
        unique_positions.insert(guard);
    }

    unique_positions.len() as u64
}

#[allow(clippy::too_many_lines)] // grandfathered in
fn part_b(input: Input) -> u64 {
    let input = input.get_original();
    // Implement solution for Part 2
    let input = input.trim();
    // All coords are (line, col)
    let mut guard_pos: (i32, i32) = input
        .lines()
        .enumerate()
        .find_map(|(i, l)| {
            l.find('^')
                .map(|v| (i.try_into().unwrap(), v.try_into().unwrap()))
        })
        .unwrap();
    let start_pos = guard_pos;
    let obstacles: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|l| {
            l.1.chars().enumerate().filter_map(move |c| {
                if c.1 == '#' {
                    Some((l.0.try_into().unwrap(), c.0.try_into().unwrap()))
                } else {
                    None
                }
            })
        })
        .collect();

    let w = input.lines().next().unwrap().len().try_into().unwrap();
    let h = input.lines().count().try_into().unwrap();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut facing = (-1i32, 0i32);

    let mut possible_loopers: Vec<(i32, i32)> = Vec::new();

    let row_obs: Box<[Box<[i32]>]> = (0..h)
        .map(|i| {
            obstacles
                .iter()
                .copied()
                .filter(|o| o.0 == i)
                .map(|o| o.1)
                .collect()
        })
        .collect();
    let col_obs: Box<[Box<[i32]>]> = (0..w)
        .map(|i| {
            obstacles
                .iter()
                .copied()
                .filter(|o| o.1 == i)
                .map(|o| o.0)
                .collect()
        })
        .collect();

    while guard_pos.0 >= 0 && guard_pos.1 >= 0 && guard_pos.0 < h && guard_pos.1 < w {
        visited.insert((
            guard_pos.0.try_into().unwrap(),
            guard_pos.1.try_into().unwrap(),
        ));
        if !obstacles.contains(&(guard_pos.0 + facing.0, guard_pos.1 + facing.1)) {
            let gp0: usize = guard_pos.0.try_into().unwrap();
            let gp1: usize = guard_pos.1.try_into().unwrap();
            let obstacle_to_right = if facing.0 > 0 {
                // facing down
                row_obs[gp0].iter().any(|i| *i < guard_pos.1)
            } else if facing.0 < 0 {
                // facing up
                row_obs[gp0].iter().any(|i| *i > guard_pos.1)
            } else if facing.1 > 0 {
                // facing right
                col_obs[gp1].iter().any(|i| *i > guard_pos.0)
            } else {
                // facing left
                col_obs[gp1].iter().any(|i| *i < guard_pos.0)
            };
            if obstacle_to_right {
                possible_loopers.push((guard_pos.0 + facing.0, guard_pos.1 + facing.1));
            }
            guard_pos = (guard_pos.0 + facing.0, guard_pos.1 + facing.1);
            continue;
        }
        facing = match facing {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => unreachable!(),
        };
    }
    possible_loopers.sort_unstable();
    possible_loopers.dedup();
    possible_loopers
        .into_iter()
        .filter(|&p| {
            let mut gp = start_pos;
            let mut obstacles = obstacles.clone();
            obstacles.insert(p);
            let mut facing = (-1, 0);
            let mut visited: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
            while gp.0 >= 0 && gp.1 >= 0 && gp.0 < h && gp.1 < w {
                if let Some(v) = visited.get(&gp)
                    && v.contains(&facing)
                {
                    return true;
                }
                let ent = visited.entry(gp).or_insert(Vec::with_capacity(1));
                if !ent.contains(&facing) {
                    ent.push(facing);
                }
                if !obstacles.contains(&(gp.0 + facing.0, gp.1 + facing.1)) {
                    gp = (gp.0 + facing.0, gp.1 + facing.1);
                    continue;
                }
                facing = match facing {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                };
            }
            false
        })
        .filter(|&(y, x)| x >= 0 && x < w && y >= 0 && y < h)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = aoc_helpers::mk_test_input!(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        );
        assert_eq!(part_a(input), 41);
    }
    #[test]
    fn part_b_works() {
        let input = aoc_helpers::mk_test_input!(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        );
        assert_eq!(part_b(input), 6);
    }
}
