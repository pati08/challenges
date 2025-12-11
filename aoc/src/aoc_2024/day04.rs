use std::collections::HashSet;

use challenges_input::Input;

const DIR_VEC: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const DIR_VEC_DIAGS_ONLY: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    const SEARCH_STR: &str = "XMAS";
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                for dir in 0..8 {
                    if search(SEARCH_STR, &grid, r, c, dir) {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn search(search_str: &str, grid: &[Vec<char>], from_r: usize, from_c: usize, dir: usize) -> bool {
    let (r_mod, c_mod) = DIR_VEC[dir];

    for i in 0..search_str.len() {
        let r = i32::try_from(from_r).unwrap() + r_mod * i32::try_from(i).unwrap();
        let c = i32::try_from(from_c).unwrap() + c_mod * i32::try_from(i).unwrap();
        if r < 0
            || c < 0
            || r >= i32::try_from(grid.len()).unwrap()
            || c >= i32::try_from(grid[0].len()).unwrap()
            || grid[usize::try_from(r).unwrap()][usize::try_from(c).unwrap()]
                != search_str.chars().nth(i).unwrap()
        {
            return false;
        }
    }
    true
}

fn search_b(
    search_str: &str,
    grid: &[Vec<char>],
    from_r: usize,
    from_c: usize,
    dir: usize,
) -> bool {
    let (r_mod, c_mod) = DIR_VEC[dir];

    for i in 0..search_str.len() {
        let r = i32::try_from(from_r).unwrap() + r_mod * i32::try_from(i).unwrap();
        let c = i32::try_from(from_c).unwrap() + c_mod * i32::try_from(i).unwrap();
        if r < 0
            || c < 0
            || r >= i32::try_from(grid.len()).unwrap()
            || c >= i32::try_from(grid[0].len()).unwrap()
            || grid[usize::try_from(r).unwrap()][usize::try_from(c).unwrap()]
                != search_str.chars().nth(i).unwrap()
        {
            return false;
        }
    }
    true
}

fn part_b(input: &Input) -> u64 {
    const SEARCH_STR: &str = "MAS";

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut centers: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'M' {
                for (dir, (r_mod, c_mod)) in DIR_VEC_DIAGS_ONLY.iter().enumerate() {
                    if search_b(SEARCH_STR, &grid, r, c, dir) {
                        let r = i32::try_from(r).unwrap() + *r_mod;
                        let c = i32::try_from(c).unwrap() + *c_mod;
                        let center = (usize::try_from(r).unwrap(), usize::try_from(c).unwrap());
                        if centers.contains(&center) {
                            sum += 1;
                        } else {
                            centers.insert(center);
                        }
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use aoc_helpers::mk_test_input;

    use super::*;

    #[test]
    fn part_a_works() {
        let input = mk_test_input!(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        );
        assert_eq!(part_a(&input), 18);
    }

    #[test]
    fn part_b_works() {
        let input = mk_test_input!(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        );
        assert_eq!(part_b(&input), 9);
    }
}
