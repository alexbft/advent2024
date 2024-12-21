use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    solve_n(input, 25)
}

fn solve_n(input: &str, n: usize) -> usize {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    let graph0 = Graph::from_edges(&[
        ('0', '>', 'A'),
        ('0', '^', '2'),
        ('A', '^', '3'),
        ('1', '>', '2'),
        ('1', '^', '4'),
        ('2', '>', '3'),
        ('2', '^', '5'),
        ('3', '^', '6'),
        ('4', '>', '5'),
        ('4', '^', '7'),
        ('5', '>', '6'),
        ('5', '^', '8'),
        ('6', '^', '9'),
        ('7', '>', '8'),
        ('8', '>', '9'),
    ]);

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    let graph1 = Graph::from_edges(&[
        ('<', '>', 'v'),
        ('v', '>', '>'),
        ('v', '^', '^'),
        ('>', '^', 'A'),
        ('^', '>', 'A'),
    ]);

    let vertices = ['<', '>', '^', 'v', 'A'];
    let mut click_costs = HashMap::new();
    for &v1 in &vertices {
        for &v2 in &vertices {
            click_costs.insert((v1, v2), 1usize);
        }
    }
    for _i in 0..n {
        click_costs = graph1.calc_click_costs(&click_costs);
    }
    click_costs = graph0.calc_click_costs(&click_costs);

    let mut result = 0;
    for line in input.lines() {
        let num: usize = line[..3].parse().unwrap();
        let mut prev = 'A';
        for c in line.chars() {
            result += click_costs[&(prev, c)] * num;
            prev = c;
        }
    }
    result
}

type Edges = HashMap<char, Vec<(char, char)>>;

struct Graph {
    edges: Edges,
}

impl Graph {
    fn from_edges(edges_list: &[(char, char, char)]) -> Self {
        let mut edges: Edges = HashMap::new();
        for &(from, dir, to) in edges_list {
            edges.entry(from).or_default().push((to, dir));
            edges.entry(to).or_default().push((from, opposite(dir)));
        }
        Graph { edges }
    }

    // calculates how much it costs to go from previous button to a given button and click it
    fn calc_click_costs(
        &self,
        click_costs: &HashMap<(char, char), usize>, // this is click costs of a parent controller
    ) -> HashMap<(char, char), usize> {
        let vertices = ['<', '>', '^', 'v', 'A'];
        // Given a graph where nodes are pairs of (current button of this controller, current button of the parent controller)
        // Let's calculate the shortest paths between all pairs of nodes
        let mut paths = HashMap::new();
        let mut path_vertices = HashSet::new();
        for (&from, edges) in self.edges.iter() {
            for &prev in &vertices {
                path_vertices.insert((from, prev));
                for &(to, dir) in edges {
                    paths.insert(((from, prev), (to, dir)), click_costs[&(prev, dir)]);
                }
            }
        }
        // Use Floyd-Warshall algorithm to calculate the shortest paths
        for &mid in &path_vertices {
            for &a in &path_vertices {
                for &b in &path_vertices {
                    if paths.contains_key(&(a, mid)) && paths.contains_key(&(mid, b)) {
                        let cost = paths[&(a, mid)] + paths[&(mid, b)];
                        let cur = paths.entry((a, b)).or_insert(cost);
                        if *cur > cost {
                            *cur = cost;
                        }
                    }
                }
            }
        }
        let mut new_click_costs = HashMap::new();
        for &from in self.edges.keys() {
            for &to in self.edges.keys() {
                // Assume parent controller is at 'A'
                if from == to {
                    // To click the same button, we just click 'A' on parent controller.
                    // It always costs 1.
                    new_click_costs.insert((from, to), 1);
                    continue;
                }
                // The cost to click the button 'to' on this controller equals to shortest path from (from, 'A') to (to, {x}),
                // then the cost to click 'A' from button {x} on the parent controller.
                let min_cost = vertices.iter().filter_map(|&v| {
                    let key = ((from, 'A'), (to, v));
                    paths.get(&key).map(|&cost| cost + click_costs[&(v, 'A')])
                }).min().unwrap();
                new_click_costs.insert((from, to), min_cost);
            }
        }
        new_click_costs
    }
}

fn opposite(dir: char) -> char {
    match dir {
        '<' => '>',
        '>' => '<',
        '^' => 'v',
        'v' => '^',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            029A
            980A
            179A
            456A
            379A
        "};
        assert_eq!(solve_n(input, 2), 126384);
    }
}
