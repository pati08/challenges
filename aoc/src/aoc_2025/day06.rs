use challenges_input::Input;

pub const TRIM: bool = false;

pub fn run(input: Input) -> String {
    aoc_helpers::ref_run(input, part_a, part_b)
}

fn part_a(input: &Input) -> u64 {
    let nums: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.trim().to_string())
        .take_while(|l| l.chars().next().unwrap().is_ascii_digit())
        .map(|i| {
            i.split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|i| {
            let operands = (0..nums.len()).map(|j| nums[j][i.0]);
            let operation = match i.1 {
                "+" => u64::strict_add,
                "*" => u64::strict_mul,
                _ => unreachable!(),
            };
            let start = match i.1 {
                "+" => 0,
                "*" => 1,
                _ => unreachable!(),
            };
            operands.fold(start, operation)
        })
        .sum()
}

fn part_b(input: &Input) -> u64 {
    let input_grid: Vec<Vec<_>> = input.lines().map(|i| i.chars().collect()).collect();
    let ops_line = input.lines().last().unwrap();
    let col_h = input.lines().count() - 1;
    let mut ops_info: Vec<(char, usize)> = Vec::new();
    let mut blanks_count = 0;
    for c in ops_line.chars() {
        if c.is_ascii_whitespace() {
            blanks_count += 1;
        } else {
            if blanks_count > 0 {
                ops_info.iter_mut().last().unwrap().1 = blanks_count - 1;
            }
            ops_info.push((c, 0));
            blanks_count = 0;
        }
    }
    ops_info.iter_mut().last().unwrap().1 = blanks_count;

    ops_info
        .into_iter()
        .scan(0usize, |acc, (op, width)| {
            let operation = match op {
                '+' => u64::strict_add,
                '*' => u64::strict_mul,
                _ => unreachable!(),
            };
            let start = match op {
                '+' => 0,
                '*' => 1,
                _ => unreachable!(),
            };

            let operands = (*acc..=width + *acc).map(|c| {
                (0..col_h)
                    .filter_map(|r| {
                        let c = input_grid[r][c];
                        if c.is_ascii_digit() { Some(c) } else { None }
                    })
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            });
            *acc += width + 2;
            Some(operands.fold(start, operation))
        })
        .sum()
}

aoc_helpers::mk_aoc_test!(
    &"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    4277556,
    3263827
);
