use std::collections::HashMap;

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

    let mut solver = Solver::new(&graph0, &graph1, n);

    let mut result = 0;
    for line in input.lines() {
        let num: usize = line[..3].parse().unwrap();
        let mut prev = 'A';
        for c in line.chars() {
            result += solver.get_click_cost(prev, c, 0) * num;
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
}

struct Solver<'a> {
    graph0: &'a Graph,
    graph1: &'a Graph,
    cache: HashMap<(char, char, usize), usize>,
    depth: usize,
}

impl<'a> Solver<'a> {
    fn new(graph0: &'a Graph, graph1: &'a Graph, depth: usize) -> Self {
        Solver {
            graph0,
            graph1,
            cache: HashMap::new(),
            depth,
        }
    }

    // cost to go from 'from' to 'to' and click 'to'
    fn get_click_cost(&mut self, from: char, to: char, depth: usize) -> usize {
        if depth > self.depth {
            return 1;
        }
        if let Some(&cost) = self.cache.get(&(from, to, depth)) {
            return cost;
        }
        let graph = if depth == 0 { self.graph0 } else { self.graph1 };
        // Consider a graph with vertices as (this controller position, parent controller position).
        // We need to find the shortest path from 'from' to 'to' in this graph.
        //
        // All controllers start at 'A'. After each click on this controller, the parent controller
        // returns to 'A'.
        //
        // So, the start position is (from, 'A').
        //
        // While this controller ends at 'to', the parent controller can end at any position.
        // When calculating the final cost, we need to select the minimum cost among all possible parent controller positions.
        let start = (from, 'A');
        // Find the shortest paths from 'start' to all other vertices using Dijkstra's algorithm.
        let mut dist = HashMap::new();
        let mut estimates = HashMap::new();
        estimates.insert(start, 0);
        while !estimates.is_empty() {
            let (&min_key, &cur_dist) = estimates.iter().min_by_key(|&(_, &cost)| cost).unwrap();
            dist.insert(min_key, cur_dist);
            estimates.remove(&min_key);
            let (from, parent_from) = min_key;
            for &(to, parent_to) in &graph.edges[&from] {
                if dist.contains_key(&(to, parent_to)) {
                    continue;
                }
                // To go through this edge, we need to move parent controller to 'parent_to' and click it.
                // If there is no parent controller, the cost is 1.
                let dist_to = cur_dist + self.get_click_cost(parent_from, parent_to, depth + 1);
                let estimated_dist = estimates.entry((to, parent_to)).or_insert(dist_to);
                if *estimated_dist > dist_to {
                    *estimated_dist = dist_to;
                }
            }
        }
        let mut min_cost = HashMap::new();
        for (&(end, parent_end), &path_cost) in dist.iter() {
            // Cost equals to path cost + click cost.
            // Click cost is a cost to move parent controller from 'parent_end' to 'A' and click it.
            let click_cost = path_cost + self.get_click_cost(parent_end, 'A', depth + 1);
            // Find the minimum click cost among all possible parent controller positions.
            let min_click_cost = min_cost.entry(end).or_insert(click_cost);
            if *min_click_cost > click_cost {
                *min_click_cost = click_cost;
            }
        }
        for (&end, &cost) in min_cost.iter() {
            self.cache.insert((from, end, depth), cost);
        }
        self.cache[&(from, to, depth)]
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
