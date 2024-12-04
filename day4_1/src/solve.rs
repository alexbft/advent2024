use lazy_static::lazy_static;

lazy_static! {
    static ref DIRS: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    static ref XMAS: Vec<char> = "XMAS".chars().collect();
}

pub fn solve(input: String) -> usize {
    let rows: Vec<_> = input.lines().collect();
    let w = rows[0].len();
    let h = rows.len();
    let chars: Vec<char> = rows.join("").chars().collect();
    (0..w * h)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            DIRS.iter()
                .filter(|(dx, dy)| check_xmas(&chars, w, h, x, y, *dx, *dy))
                .count()
        })
        .sum()
}

fn get_char(chars: &Vec<char>, w: usize, h: usize, x: i32, y: i32) -> char {
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return ' ';
    }
    chars[y as usize * w + x as usize]
}

fn check_xmas(chars: &Vec<char>, w: usize, h: usize, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    (0..=3).all(|i| {
        let nx = x as i32 + dx * i;
        let ny = y as i32 + dy * i;
        get_char(chars, w, h, nx, ny) == XMAS[i as usize]
    })
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "}
        .to_string();
        assert_eq!(solve(input), 18);
    }
}
