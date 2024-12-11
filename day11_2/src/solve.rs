use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    solve_n(input, 75)
}

fn solve_n(input: &str, n: i32) -> usize {
    let mut nums: HashMap<i64, usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .counts();
    (0..n).for_each(|_| {
        nums = transform(&nums);
    });
    nums.values().sum()
}

fn transform(nums: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut result = HashMap::new();
    for (&k, &v) in nums.iter() {
        if k == 0 {
            *result.entry(1i64).or_insert(0usize) += v;
            continue;
        }
        let s = k.to_string();
        if s.len() % 2 == 0 {
            let left = s[..s.len() / 2].parse().unwrap();
            let right = s[s.len() / 2..].parse().unwrap();
            *result.entry(left).or_insert(0usize) += v;
            *result.entry(right).or_insert(0usize) += v;
            continue;
        }
        *result.entry(k * 2024).or_insert(0usize) += v;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "125 17";
        assert_eq!(solve_n(input, 25), 55312usize);
    }
}
