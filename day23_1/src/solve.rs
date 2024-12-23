use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();
    for line in input.lines() {
        let mut parts = line.splitn(2, '-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        vertices.insert(from);
        vertices.insert(to);
        edges.insert((from, to));
        edges.insert((to, from));
    }
    let vt: Vec<_> = vertices.iter().filter(|&v| v.starts_with('t')).collect();
    let mut triples = HashSet::new();
    for &(from, to) in &edges {
        for &&v in &vt {
            if edges.contains(&(from, v)) && edges.contains(&(v, to)) {
                let mut triple = vec![from, v, to];
                triple.sort();
                triples.insert(triple);
            }
        }
    }
    triples.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
        "};
        assert_eq!(solve(input), 7);
    }
}
