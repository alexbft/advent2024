use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = chars.len() as i32;
    let w = chars[0].len() as i32;
    let mut crawler = Crawler::create(chars);
    let mut result = 0usize;
    for y in 0..h {
        for x in 0..w {
            if !crawler.visited.contains(&(x, y)) {
                let (area, perimeter) = crawler.crawl(x, y);
                result += area * perimeter;
            }
        }
    }
    result
}

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Crawler {
    chars: Vec<Vec<char>>,
    visited: HashSet<(i32, i32)>,
}

impl Crawler {
    fn create(chars: Vec<Vec<char>>) -> Self {
        Self {
            chars,
            visited: HashSet::new(),
        }
    }

    fn get_at(&self, x: i32, y: i32) -> Option<char> {
        self.chars
            .get(y as usize)
            .and_then(|row| row.get(x as usize).copied())
    }

    fn dfs(&mut self, x: i32, y: i32, area: &mut usize, perimeter: &mut usize) {
        if self.visited.contains(&(x, y)) {
            return;
        }
        self.visited.insert((x, y));
        *area += 1;
        let ch = self.get_at(x, y).unwrap();
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if self.get_at(nx, ny) == Some(ch) {
                self.dfs(nx, ny, area, perimeter);
            } else {
                *perimeter += 1;
            }
        }
    }

    fn crawl(&mut self, x: i32, y: i32) -> (usize, usize) {
        let mut area = 0usize;
        let mut perimeter = 0usize;
        self.dfs(x, y, &mut area, &mut perimeter);
        (area, perimeter)
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
        assert_eq!(solve(input), 1930usize);
    }
}
