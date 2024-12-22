pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut num: u64 = line.parse().unwrap();
            for _i in 0..2000 {
                num = transform(num);
            }
            println!("{}: {}", line, num);
            num
        })
        .sum()
}

fn transform(num: u64) -> u64 {
    let mut num = num;
    num = (num ^ (num * 64)) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            1
            10
            100
            2024
        "};
        assert_eq!(solve(input), 37327623);
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform(123), 15887950);
        assert_eq!(transform(15887950), 16495136);
    }
}
