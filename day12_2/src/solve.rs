use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = chars.len() as i32;
    let w = chars[0].len() as i32;
    let mut crawler = Crawler::create(chars);
    let mut result = 0usize;
    for y in 0..h {
        for x in 0..w {
            if !crawler.visited.contains_key(&(x, y)) {
                let (area, sides) = crawler.crawl(x, y);
                result += area * sides;
            }
        }
    }
    result
}

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Crawler {
    chars: Vec<Vec<char>>,
    visited: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

impl Crawler {
    fn create(chars: Vec<Vec<char>>) -> Self {
        Self {
            chars,
            visited: HashMap::new(),
        }
    }

    fn get_at(&self, x: i32, y: i32) -> Option<char> {
        self.chars
            .get(y as usize)
            .and_then(|row| row.get(x as usize).copied())
    }

    fn dfs(&mut self, x: i32, y: i32, area: &mut i32, sides: &mut i32) {
        if self.visited.contains_key(&(x, y)) {
            return;
        }
        *area += 1;
        let ch = self.get_at(x, y).unwrap();
        let mut cell_edges = HashSet::new();
        let mut cell_paths = Vec::new();
        let mut count_edges = 0i32;
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if self.get_at(nx, ny) == Some(ch) {
                cell_paths.push((nx, ny));
            } else {
                cell_edges.insert((dx, dy));
                count_edges += 1;
            }
        }
        self.visited
            .insert((x, y), cell_edges.iter().copied().collect());
        for pos in cell_paths.iter() {
            let duplicate_edges_opt = self.visited.get(pos);
            if let Some(duplicate_edges) = duplicate_edges_opt {
                for edge in duplicate_edges {
                    if cell_edges.contains(edge) {
                        count_edges -= 1;
                    }
                }
            }
        }
        *sides += count_edges;
        for (nx, ny) in cell_paths {
            self.dfs(nx, ny, area, sides);
        }
    }

    fn crawl(&mut self, x: i32, y: i32) -> (usize, usize) {
        let mut area = 0;
        let mut sides = 0;
        self.dfs(x, y, &mut area, &mut sides);
        // println!(
        //     "ch: {}, area: {}, sides: {}",
        //     self.get_at(x, y).unwrap(),
        //     area,
        //     sides
        // );
        (area as usize, sides as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        assert_eq!(solve(input), 1206usize);
    }

    #[test]
    fn test_2() {
        let input = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        assert_eq!(solve(input), 80usize);
    }
}
