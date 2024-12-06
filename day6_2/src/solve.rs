use std::collections::HashSet;

pub fn solve(input: String) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (sx, sy) = chars.find_pos('^').unwrap();
    let mut total = 0;
    for extra_y in 0..chars.len() {
        for extra_x in 0..chars[extra_y].len() {
            if chars[extra_y][extra_x] != '.' {
                continue;
            }
            let mut visited = HashSet::new();
            let (mut x, mut y) = (sx, sy);
            let (mut dx, mut dy) = (0, -1);
            loop {
                let state = (x, y, dx, dy);
                if visited.contains(&state) {
                    total += 1;
                    break;
                }
                visited.insert(state);
                let (nx, ny) = (x + dx, y + dy);
                if (nx as usize) == extra_x && (ny as usize) == extra_y {
                    (dx, dy) = turn_right(dx, dy);
                    continue;
                }
                match chars.get_at(nx, ny) {
                    Some('#') => {
                        (dx, dy) = turn_right(dx, dy);
                    }
                    Some(_) => {
                        (x, y) = (nx, ny);
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
    total
}

fn turn_right(dx: i32, dy: i32) -> (i32, i32) {
    match (dx, dy) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

trait Matrix {
    fn get_at(&self, x: i32, y: i32) -> Option<&char>;
    fn find_pos(&self, c: char) -> Option<(i32, i32)>;
}

impl Matrix for Vec<Vec<char>> {
    fn get_at(&self, x: i32, y: i32) -> Option<&char> {
        self.get(y as usize).and_then(|row| row.get(x as usize))
    }
    fn find_pos(&self, c: char) -> Option<(i32, i32)> {
        let mut x = 0usize;
        let y = self
            .iter()
            .position(|row| match row.iter().position(|&cell| cell == c) {
                Some(x_) => {
                    x = x_;
                    return true;
                }
                None => false,
            });
        match y {
            Some(y_) => Some((x as i32, y_ as i32)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "}
        .to_string();
        assert_eq!(solve(input), 6);
    }
}
