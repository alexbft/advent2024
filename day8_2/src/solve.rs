use itertools::Itertools;
use std::collections::HashMap;

type Point = (usize, usize);

pub fn solve(input: String) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut char_positions: HashMap<char, Vec<Point>> = HashMap::new();
    chars.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &c)| {
            if c != '.' {
                char_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        });
    });
    let width = chars[0].len() as i64;
    let height = chars.len() as i64;
    char_positions
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .tuple_combinations()
                .flat_map(|(p0, p1)| calc_all_positions(p0, p1, width, height))
        })
        .unique()
        .count()
}

fn calc_all_positions(p0: &Point, p1: &Point, width: i64, height: i64) -> Vec<(i64, i64)> {
    let dx = p1.0 as i64 - p0.0 as i64;
    let dy = p1.1 as i64 - p0.1 as i64;
    let extrapolate = |x0: i64, y0: i64, dx: i64, dy: i64| {
        (1..)
            .map(move |i| (x0 + i * dx, y0 + i * dy))
            .take_while(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
    };
    extrapolate(p0.0 as i64, p0.1 as i64, dx, dy)
        .chain(extrapolate(p1.0 as i64, p1.1 as i64, -dx, -dy))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "}
        .to_string();
        assert_eq!(solve(input), 34usize);
    }
}
