pub fn solve(input: String) -> i64 {
    let mut sum = 0i64;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let a: i64 = parts.next().unwrap().parse().unwrap();
        let xs: Vec<i64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if try_equation(a, &xs) {
            sum += a;
        }
    }
    sum
}

fn try_equation(a: i64, xs: &[i64]) -> bool {
    fn rec(a: i64, b: i64, xs: &[i64]) -> bool {
        if b > a {
            return false;
        }
        if xs.is_empty() {
            return a == b;
        }
        rec(a, b * xs[0], &xs[1..]) || rec(a, b + xs[0], &xs[1..])
    }

    assert_ne!(xs.len(), 0);
    rec(a, xs[0], &xs[1..])
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "}
        .to_string();
        assert_eq!(solve(input), 3749i64);
    }
}
