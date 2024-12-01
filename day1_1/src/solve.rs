use itertools::Itertools;
pub fn solve(input: String) -> i32 {
    let input_lines = input.lines();
    let num_pairs = input_lines
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut num_lists = transpose(num_pairs);
    for list in &mut num_lists {
        list.sort()
    }
    let mut sum = 0;
    for pair in transpose(num_lists) {
        let (a, b) = pair.iter().collect_tuple().unwrap();
        sum += (a - b).abs();
    }
    sum
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "}
        .to_string();
        assert_eq!(solve(input), 11);
    }
}
