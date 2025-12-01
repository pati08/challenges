use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}
fn part_a(input: Input) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let w = grid[0].len();
    let h = grid.len();

    (0..w)
        .flat_map(|x| {
            let grid = &grid;
            (0..h).filter(move |&y| {
                !((0..x).any(|x2| grid[y][x2] >= grid[y][x])
                    && (x + 1..w).any(|x2| grid[y][x2] >= grid[y][x])
                    && (0..y).any(|y2| grid[y2][x] >= grid[y][x])
                    && (y + 1..h).any(|y2| grid[y2][x] >= grid[y][x]))
            })
        })
        .count() as u64
}

fn part_b(input: Input) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let w = grid[0].len();
    let h = grid.len();

    (1..w - 1)
        .flat_map(|x| {
            let grid = &grid;
            (1..h - 1).map(move |y| {
                let res = (0..x)
                    .rev()
                    .take_while(|&x2| grid[y][x2] < grid[y][x])
                    .count()
                    * ((x + 1..w)
                        .take_while(|&x2| grid[y][x2] < grid[y][x])
                        .count()
                        + 1)
                    * ((0..y)
                        .rev()
                        .take_while(|&y2| grid[y2][x] < grid[y][x])
                        .count()
                        + 1)
                    * ((y + 1..h)
                        .take_while(|&y2| grid[y2][x] < grid[y][x])
                        .count()
                        + 1);
                dbg!(x, y);
                dbg!(res)
            })
        })
        .max()
        .unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "30373
25512
65332
33549
35390"
        );
        assert_eq!(part_a(input), 21);
    }

    #[test]
    fn test_part_b() {
        let input = mk_test_input!(
            "30373
25512
65332
33549
35390"
        );
        assert_eq!(part_b(input), 8);
    }
}
