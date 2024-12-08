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
            positions.iter().permutations(2).filter_map(|perm| {
                let (x0, y0) = perm[0];
                let (x1, y1) = perm[1];
                let x2 = 2 * (*x1 as i64) - (*x0 as i64);
                let y2 = 2 * (*y1 as i64) - (*y0 as i64);
                if x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                    Some((x2, y2))
                } else {
                    None
                }
            })
        })
        .unique()
        .count()
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
        assert_eq!(solve(input), 14usize);
    }
}
