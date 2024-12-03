use itertools::Itertools;
use regex::Regex;

pub fn solve(input: String) -> i64 {
    let mut sum = 0i64;
    let mut enabled = true;
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for capture in mul_regex.captures_iter(&input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let (a, b) = capture
                        .iter()
                        .skip(1)
                        .map(|m| m.unwrap().as_str().parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    sum += a * b;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;

    #[test]
    fn test_solve() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(solve(input), 48i64);
    }
}
