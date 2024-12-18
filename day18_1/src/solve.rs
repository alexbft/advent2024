use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    solve_n(input, 71, 71, 1024)
}

fn solve_n(input: &str, width: i32, height: i32, take_n: usize) -> usize {
    let mut blocks = HashSet::new();
    for line in input.lines().take(take_n) {
        let x: Vec<i32> = line.splitn(2, ',').map(|s| s.parse().unwrap()).collect();
        blocks.insert((x[0], x[1]));
    }
    let start = (0, 0);
    let end = (width - 1, height - 1);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return steps;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for dir in &dirs {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
                continue;
            }
            if blocks.contains(&next) {
                continue;
            }
            queue.push_back((next, steps + 1));
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
        "};
        assert_eq!(solve_n(input, 7, 7, 12), 22);
    }
}
