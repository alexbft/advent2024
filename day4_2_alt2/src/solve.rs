pub fn solve(input: String) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let w = chars[0].len();

    (0..chars.len())
        .map(|y| {
            (0..w)
                .filter(|&x| check_mas(&chars, x, y, 1, 1) && check_mas(&chars, x + 2, y, -1, 1))
                .count()
        })
        .sum()
}

fn check_mas(chars: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let seq: String = (0..3)
        .map(|i| {
            let nx = (x as i32 + dx * i) as usize;
            let ny = (y as i32 + dy * i) as usize;
            chars
                .get(ny)
                .and_then(|row| row.get(nx))
                .copied()
                .unwrap_or(' ')
        })
        .collect();
    seq == "MAS" || seq == "SAM"
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
