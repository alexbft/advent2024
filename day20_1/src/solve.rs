use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> usize {
    solve_all(input).into_iter().filter(|&x| x >= 100).count()
}

fn solve_all(input: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    let start = find_pos(&grid, 'S');
    let base_result = shortest_path(&grid, &start, (0, 0), (0, 0)).len();
    let mut check_path = |x1: usize, y1: usize, x2: usize, y2: usize| {
        if x2 <= 0 || y2 <= 0 || x2 >= width - 1 || y2 >= height - 1 || grid[y2][x2] == '#' {
            return;
        }
        let p1 = (x1, y1);
        let p2 = (x2, y2);
        let path = shortest_path(&grid, &start, p1, p2);
        for (&a, &b) in path.iter().tuple_windows() {
            if a == p1 && b == p2 {
                let score = base_result - path.len();
                result.push(score);
                break;
            }
        }
    };
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[y][x] == '#' {
                check_path(x, y, x - 1, y);
                check_path(x, y, x, y - 1);
                check_path(x, y, x + 1, y);
                check_path(x, y, x, y + 1);
            }
        }
    }
    result.into_iter().filter(|&x| x > 0).collect()
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

fn shortest_path(
    grid: &Vec<Vec<char>>,
    start: &(usize, usize),
    ignore1: (usize, usize),
    ignore2: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    visited.insert(*start, *start);
    let dirs = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    let mut end = None;
    while let Some((x, y)) = queue.pop_front() {
        if grid[y][x] == 'E' {
            end = Some((x, y));
            break;
        }
        for (dx, dy) in dirs.iter() {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if visited.contains_key(&(new_x, new_y))
                || (grid[new_y][new_x] == '#'
                    && (new_x, new_y) != ignore1
                    && (new_x, new_y) != ignore2)
            {
                continue;
            }
            visited.insert((new_x, new_y), (x, y));
            queue.push_back((new_x, new_y));
        }
    }
    if let Some(end) = end {
        let mut path = Vec::new();
        let mut current = end;
        while current != *start {
            path.push(current);
            current = visited[&current];
        }
        path.reverse();
        path
    } else {
        panic!("No path")
    }
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
