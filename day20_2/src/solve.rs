use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> usize {
    solve_all(input).into_iter().filter(|&x| x >= 100).count()
}

fn solve_all(input: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    let all_paths_start = find_all_paths(&grid, &start);
    let all_paths_end = find_all_paths(&grid, &end);
    let base_result = all_paths_start[&end];
    for (pos_enter, start_len) in all_paths_start.iter() {
        for (pos_exit, end_len) in all_paths_end.iter() {
            if *pos_enter == *pos_exit {
                continue;
            }
            let cheat_len = manhattan_distance(pos_enter, pos_exit);
            if cheat_len > 20 {
                continue;
            }
            let len = start_len + end_len + cheat_len;
            if len < base_result {
                result.push(base_result - len);
            }
        }
    }
    result
}

fn find_pos(grid: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == target {
                return (x, y);
            }
        }
    }
    panic!("Target not found");
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn find_all_paths(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    visited.insert(*start, 0);
    let dirs = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    while let Some((x, y)) = queue.pop_front() {
        let dist = visited[&(x, y)];
        for (dx, dy) in dirs.iter() {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if visited.contains_key(&(new_x, new_y)) || grid[new_y][new_x] == '#' {
                continue;
            }
            visited.insert((new_x, new_y), dist + 1);
            queue.push_back((new_x, new_y));
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        "};
        let expected: HashMap<usize, usize> = HashMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]);
        let actual = solve_all(input).into_iter().filter(|&x| x >= 50).counts();
        assert_eq!(actual, expected);
    }
}
