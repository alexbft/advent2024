pub fn solve(input: String) -> usize {
    let rows: Vec<_> = input.lines().collect();
    let w = rows[0].len();
    let h = rows.len();
    let chars: Vec<char> = rows.join("").chars().collect();
    // (dir) (position delta + direction of first possible match) (pos + dir of second match)
    let dirs = vec![
        ((1, 1), (2, 0), (-1, 1), (0, 2), (1, -1)),
        ((-1, -1), (-2, 0), (1, -1), (0, -2), (-1, 1)),
        ((1, -1), (2, 0), (-1, -1), (0, -2), (1, 1)),
        ((-1, 1), (-2, 0), (1, 1), (0, 2), (-1, -1)),
    ];
    let total: usize = (0..w * h)
        .map(|i| {
            let (x, y) = ((i % w) as i32, (i / w) as i32);
            dirs.iter()
                .filter(|((dx, dy), (x1, y1), (dx1, dy1), (x2, y2), (dx2, dy2))| {
                    let (xx1, yy1) = (x + x1, y + y1);
                    let (xx2, yy2) = (x + x2, y + y2);
                    check_mas(&chars, w, h, x, y, *dx, *dy)
                        && (check_mas(&chars, w, h, xx1, yy1, *dx1, *dy1)
                            || check_mas(&chars, w, h, xx2, yy2, *dx2, *dy2))
                })
                .count()
        })
        .sum();
    total / 2
}

fn get_char(chars: &Vec<char>, w: usize, h: usize, x: i32, y: i32) -> char {
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return ' ';
    }
    chars[y as usize * w + x as usize]
}

fn check_mas(chars: &Vec<char>, w: usize, h: usize, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let mas: [char; 3] = ['M', 'A', 'S'];
    (0..=2).all(|i| {
        let nx = x + dx * i;
        let ny = y + dy * i;
        get_char(chars, w, h, nx, ny) == mas[i as usize]
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
        assert_eq!(solve(input), 9);
    }
}
