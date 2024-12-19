pub fn solve(input: &str) -> usize {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let designs: Vec<_> = parts[0].trim().split(", ").collect();
    let mut result = 0;
    for line in parts[1].lines() {
        let mut visited = vec![0; line.len() + 1];
        visited[0] = 1;
        for i in 0..line.len() {
            let cur = visited[i];
            for design in designs.iter() {
                if line[i..].starts_with(design) {
                    let new_i = i + design.len();
                    if new_i <= visited.len() {
                        visited[new_i] += cur;
                    }
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
