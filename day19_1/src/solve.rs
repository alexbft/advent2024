use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let designs: Vec<_> = parts[0].trim().split(", ").collect();
    let mut result = 0;
    for line in parts[1].lines() {
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut visited = HashSet::new();
        while let Some(i) = queue.pop_front() {
            if visited.contains(&i) {
                continue;
            }
            visited.insert(i);
            for design in designs.iter() {
                if line[i..].starts_with(design) {
                    queue.push_back(i + design.len());
                }
            }
        }
        if visited.contains(&line.len()) {
            result += 1;
        }
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
        assert_eq!(solve(input), 6);
    }
}
