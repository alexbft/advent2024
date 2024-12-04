pub fn solve(input: String) -> usize {
    let rows: Vec<_> = input.lines().collect();
    let w = rows[0].len();
    let h = rows.len();
    let chars: Vec<char> = rows.join("").chars().collect();

    (0..w * h)
        .filter(|i| {
            let (x, y) = ((i % w) as i32, (i / w) as i32);
            check_mas(&chars, w, h, x, y, 1, 1) && check_mas(&chars, w, h, x + 2, y, -1, 1)
        })
        .count()
}

fn get_char(chars: &Vec<char>, w: usize, h: usize, x: i32, y: i32) -> char {
    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
        return ' ';
    }
    chars[y as usize * w + x as usize]
}

fn check_mas(chars: &Vec<char>, w: usize, h: usize, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let mas: [char; 3] = ['M', 'A', 'S'];
    let sam: [char; 3] = ['S', 'A', 'M'];
    for pattern in [mas, sam] {
        if (0..=2).all(|i| {
            let nx = x + dx * i;
            let ny = y + dy * i;
            let c = get_char(chars, w, h, nx, ny);
            c == pattern[i as usize]
        }) {
            return true;
        }
    }
    false
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
