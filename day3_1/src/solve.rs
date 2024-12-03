use regex::Regex;

pub fn solve(input: String) -> i64 {
    let mut sum = 0i64;
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for capture in mul_regex.captures_iter(&input) {
        let (_, [a_s, b_s]) = capture.extract();
        let a = a_s.parse::<i64>().unwrap();
        let b = b_s.parse::<i64>().unwrap();
        sum += a * b;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;

    #[test]
    fn test_solve() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(solve(input), 161i64);
    }
}
