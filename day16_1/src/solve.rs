use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (sx, sy) = find_pos(&grid, 'S');
    let (ex, ey) = find_pos(&grid, 'E');
    grid[sy as usize][sx as usize] = '.';
    grid[ey as usize][ex as usize] = '.';
    let mut queue = BinaryHeap::new();
    queue.push(QueueItem {
        pos: (sx, sy),
        dir: 0,
        dist: 0,
    });
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let head = queue.pop().unwrap();
        if head.pos == (ex, ey) {
            return head.dist as usize;
        }
        if !visited.insert((head.pos, head.dir)) {
            continue;
        }
        let (dx, dy) = dir_to_delta(head.dir);
        let (nx, ny) = (head.pos.0 + dx, head.pos.1 + dy);
        if grid[ny as usize][nx as usize] == '.' {
            queue.push(QueueItem {
                pos: (head.pos.0 + dx, head.pos.1 + dy),
                dir: head.dir,
                dist: head.dist + 1,
            });
        }
        queue.push(QueueItem {
            pos: head.pos,
            dir: (head.dir + 1) % 4,
            dist: head.dist + 1000,
        });
        queue.push(QueueItem {
            pos: head.pos,
            dir: (head.dir + 3) % 4,
            dist: head.dist + 1000,
        });
    }
    panic!("No path found");
}

fn find_pos(grid: &[Vec<char>], c: char) -> (i32, i32) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == c {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("Not found");
}

fn dir_to_delta(dir: i32) -> (i32, i32) {
    match dir {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

#[derive(Eq, PartialEq)]
struct QueueItem {
    pos: (i32, i32),
    dir: i32,
    dist: i32,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "};
        assert_eq!(solve(input), 7036);
    }
}
