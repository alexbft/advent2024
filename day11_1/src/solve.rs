pub fn solve(input: &str) -> usize {
    let mut nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (0..25).for_each(|_| {
        nums = transform(&nums);
    });
    nums.len()
}

fn transform(nums: &Vec<i64>) -> Vec<i64> {
    nums.iter()
        .flat_map(|&n| {
            if n == 0 {
                return vec![1i64];
            }
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let left = s[..s.len() / 2].parse().unwrap();
                let right = s[s.len() / 2..].parse().unwrap();
                return vec![left, right];
            }
            return vec![n * 2024];
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let input = vec![0i64, 1i64, 10i64, 99i64, 999i64];
        let expected = vec![1i64, 2024i64, 1i64, 0i64, 9i64, 9i64, 2021976i64];
        assert_eq!(transform(&input), expected);
    }

    #[test]
    fn test_solve() {
        let input = "125 17";
        assert_eq!(solve(input), 55312usize);
    }
}
