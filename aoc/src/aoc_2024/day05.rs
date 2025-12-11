use std::str::FromStr;

use challenges_input::Input;

pub const TRIM: bool = true;
pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

struct Rule {
    x: u64,
    y: u64,
}
impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('|');
        let (x, y) = (
            s.next().ok_or(())?.parse().map_err(|_| ())?,
            s.next().ok_or(())?.parse().map_err(|_| ())?,
        );
        Ok(Self { x, y })
    }
}

fn part_a(mut input: Input) -> u64 {
    let rules: Vec<Rule> = (0..).map_while(|_| input.next::<Rule>()).collect();
    input.skip(1);
    let updates = (0..)
        .map_while(|_| {
            Some(
                input
                    .next_line()?
                    .split(',')
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    updates
        .into_iter()
        .filter(|update| find_issue(update, &rules).is_none())
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_b(mut input: Input) -> u64 {
    let rules: Vec<Rule> = (0..).map_while(|_| input.next::<Rule>()).collect();
    input.skip(1);
    let updates = (0..)
        .map_while(|_| {
            Some(
                input
                    .next_line()?
                    .split(',')
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    updates
        .into_iter()
        .filter(|update| find_issue(update, &rules).is_some())
        .map(|update| reorder(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn find_issue(pages: &[u64], rules: &[Rule]) -> Option<(usize, usize)> {
    for (page_num, page) in pages.iter().enumerate() {
        for rule in rules.iter().filter(|rule| rule.y == *page) {
            if let Some(rule_x_idx) = pages.iter().position(|i| i == &rule.x)
                && rule_x_idx > page_num
            {
                return Some((page_num, rule_x_idx));
            }
        }
    }
    None
}

fn reorder(mut pages: Vec<u64>, rules: &[Rule]) -> Vec<u64> {
    while let Some(v) = find_issue(&pages, rules) {
        pages.swap(v.0, v.1);
    }
    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = aoc_helpers::mk_test_input!(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        );
        assert_eq!(part_a(input), 143);
    }
    #[test]
    fn part_b_works() {
        let input = aoc_helpers::mk_test_input!(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        );
        assert_eq!(part_b(input), 123);
    }
}
