use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> i32 {
    let mut total_by_diffs = HashMap::new();
    for line in input.lines() {
        let mut num: u64 = line.parse().unwrap();
        let mut a: Vec<i32> = Vec::new();
        a.push((num % 10) as i32);
        for _i in 0..2000 {
            num = transform(num);
            a.push((num % 10) as i32);
        }
        let mut all_diffs = HashSet::new();
        for w in a.windows(5) {
            let diff_vec: Vec<_> = w.windows(2).map(|w| w[1] - w[0]).collect();
            let diff = (diff_vec[0], diff_vec[1], diff_vec[2], diff_vec[3]);
            if all_diffs.insert(diff) {
                *total_by_diffs.entry(diff).or_insert(0) += w[4];
            }
        }
    }
    let max_entry = total_by_diffs.into_iter().max_by_key(|(_k, v)| *v).unwrap();
    println!("{:?}", max_entry);
    max_entry.1
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
            2
            3
            2024
        "};
        assert_eq!(solve(input), 23);
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform(123), 15887950);
        assert_eq!(transform(15887950), 16495136);
    }
}
