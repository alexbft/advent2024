use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> String {
    solve_n(input, 71, 71).unwrap_or("No solution".to_string())
}

type Point = (i32, i32);

fn solve_n(input: &str, width: i32, height: i32) -> Option<String> {
    let mut blocks = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let x: Vec<i32> = line.splitn(2, ',').map(|s| s.parse().unwrap()).collect();
        blocks.insert((x[0], x[1]), i);
    }
    let mut l: usize = 0;
    let mut r = blocks.len();
    while l < r {
        let m = (l + r) / 2;
        if check_path(&blocks, width, height, m) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let index = l;
    blocks.iter().find_map(|(k, &v)| {
        if v == index {
            Some(format!("{},{}", k.0, k.1))
        } else {
            None
        }
    })
}

fn check_path(blocks: &HashMap<Point, usize>, width: i32, height: i32, time: usize) -> bool {
    let start = (0, 0);
    let end = (width - 1, height - 1);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some(pos) = queue.pop_front() {
        if pos == end {
            return true;
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
            if let Some(block) = blocks.get(&next) {
                if *block <= time {
                    continue;
                }
            }
            queue.push_back(next);
        }
    }
    false
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
        assert_eq!(solve_n(input, 7, 7), Some("6,1".to_string()));
    }
}
