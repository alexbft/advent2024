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
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                if dy.abs() + dx.abs() != 2 {
                    continue;
                }
                let exit_x = pos_enter.0 as i32 + dx;
                let exit_y = pos_enter.1 as i32 + dy;
                let pos_exit = (exit_x as usize, exit_y as usize);
                if !all_paths_end.contains_key(&pos_exit) {
                    continue;
                }
                let end_len = all_paths_end[&pos_exit];
                let len = start_len + end_len + 2;
                if len < base_result {
                    result.push(base_result - len);
                }
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
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);
        let actual = solve_all(input).into_iter().counts();
        assert_eq!(actual, expected);
    }
}
