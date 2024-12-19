pub fn solve(input: &str) -> usize {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let designs: Vec<_> = parts[0].trim().split(", ").collect();
    let mut result = 0;
    for line in parts[1].lines() {
        let mut visited = vec![0; line.len() + 1];
        visited[0] = 1;
        for i in 0..line.len() {
            let cur = visited[i];
            if cur == 0 {
                continue;
            }
            for design in &designs {
                if line[i..].starts_with(design) {
                    visited[i + design.len()] += cur;
                }
            }
        }
        result += visited[line.len()];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        "};
        assert_eq!(solve(input), 16);
    }
}
