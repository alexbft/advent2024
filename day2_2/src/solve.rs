pub fn solve(input: String) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        if check_safe(&nums) {
            safe_count += 1;
            continue;
        }
        for i in 0..nums.len() {
            let filtered_nums: Vec<i32> = nums
                .iter()
                .enumerate()
                .filter_map(|(index, num)| if i != index { Some(*num) } else { None })
                .collect();
            if check_safe(&filtered_nums) {
                safe_count += 1;
                break;
            }
        }
    }
    safe_count
}

fn check_safe(nums: &Vec<i32>) -> bool {
    let diffs: Vec<_> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    if !(diffs.iter().all(|diff| *diff > 0) || diffs.iter().all(|diff| *diff < 0)) {
        return false;
    }
    if diffs.iter().any(|diff| diff.abs() > 3) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "}
        .to_string();
        assert_eq!(solve(input), 4);
    }
}
