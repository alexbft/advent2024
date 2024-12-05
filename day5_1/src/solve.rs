use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: String) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let (part1, part2) = lines.splitn(2, |&s| s.is_empty()).collect_tuple().unwrap();

    let rules: HashSet<(i32, i32)> = part1
        .into_iter()
        .map(|&line| {
            line.splitn(2, "|")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // process orders
    part2
        .into_iter()
        .map(|&line| -> Vec<i32> { line.split(',').map(|s| s.parse().unwrap()).collect() })
        .filter(|candidate| is_correct_order(candidate, &rules))
        .map(|order| order[order.len() / 2])
        .sum()
}

fn is_correct_order(candidate: &Vec<i32>, rules: &HashSet<(i32, i32)>) -> bool {
    candidate
        .iter()
        .tuple_combinations()
        .all(|(&a, &b)| !rules.contains(&(b, a)))
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            47|53
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
            97,13,75,29,47
        "}
        .to_string();
        assert_eq!(solve(input), 143);
    }
}
