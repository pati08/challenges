use challenges_input::Input;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

fn movable(y: usize, x: usize, grid: &[Vec<bool>]) -> bool {
    let h = grid.len();
    let w = grid[0].len();

    let x_range = (-1i64..=1)
        .filter(|i| (*i >= 0 || x > 0) && (*i <= 0 || x < w - 1))
        .map(|v| (x as i64 + v) as usize);

    let y_range = (-1i64..=1)
        .filter(|i| (*i >= 0 || y > 0) && (*i <= 0 || y < h - 1))
        .map(|v| (y as i64 + v) as usize);

    let num_around = y_range
        .flat_map(|y1| x_range.clone().map(move |x1| (y1, x1)))
        .filter(|&(y1, x1)| grid[y1][x1] && (y, x) != (y1, x1))
        .count();
    num_around < 4
}

fn part_a(input: Input) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let h = grid.len();
    let w = grid[0].len();
    (0..h)
        .flat_map(|y| (0..w).map(move |x| (y, x)))
        .filter(|&(y, x)| grid[y][x])
        .filter(|&(y, x)| movable(y, x, &grid))
        .count() as u64
}

fn part_b(input: Input) -> u64 {
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let h = grid.len();
    let w = grid[0].len();
    let mut total_removed = 0;
    let mut num_removed = 1;
    while num_removed > 0 {
        let rolls_movable = (0..h)
            .flat_map(|y| (0..w).map(move |x| (y, x)))
            .filter(|&(y, x)| grid[y][x])
            .filter(|&(y, x)| movable(y, x, &grid))
            .collect::<Vec<_>>();
        num_removed = 0;
        for (y, x) in rolls_movable {
            grid[y][x] = false;
            num_removed += 1;
            total_removed += 1;
        }
    }
    total_removed
}

aoc_helpers::mk_aoc_test!(
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    13,
    43
);
