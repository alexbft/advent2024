use std::collections::HashSet;

pub fn solve(input: &str) -> String {
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();
    let mut cliques = HashSet::new();
    for line in input.lines() {
        let mut parts = line.splitn(2, '-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        vertices.insert(from);
        vertices.insert(to);
        edges.insert((from, to));
        edges.insert((to, from));
        if from < to {
            cliques.insert(vec![from, to]);
        } else {
            cliques.insert(vec![to, from]);
        }
    }
    loop {
        let mut new_cliques = HashSet::new();
        for clique in &cliques {
            for &v in &vertices {
                if clique.iter().all(|&c| edges.contains(&(c, v))) {
                    let mut new_clique = clique.clone();
                    new_clique.push(v);
                    new_clique.sort();
                    new_cliques.insert(new_clique);
                }
            }
        }
        if new_cliques.is_empty() {
            break;
        }
        cliques = new_cliques;
    }
    assert_eq!(cliques.len(), 1);
    let result = cliques.into_iter().next().unwrap();
    result.join(",")
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
        assert_eq!(solve(input), "co,de,ka,ta");
    }
}
